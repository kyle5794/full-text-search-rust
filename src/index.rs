use croaring::Bitmap;
use std::collections::HashMap;
use crate::document::Document;
use crate::tokenizer;

pub struct Index(pub HashMap<String, Bitmap>);

impl Index {
    pub fn new() -> Index {
        Index(HashMap::new())
    }

    pub fn add(&mut self, documents: Vec<Document>) {
        for document in documents {
            for token in tokenizer::analyze(&document.text) {
                match self.0.get_mut(&token) {
                    Some(ids) => ids.add(document.id),
                    None => {
                        let mut s = Bitmap::create();
                        s.add(document.id);
                        self.0.insert(token, s);
                    }
                };
            }
        }
    }

    pub fn add_token(&mut self, token: String, id: u32) {
        match self.0.get_mut(&token) {
            Some(ids) => ids.add(id),
            None => {
                let mut s = Bitmap::create();
                s.add(id);
                self.0.insert(token, s);
            }
        }
    }

    pub fn search(&self, text: &str) -> Option<Vec<u32>> {
        let mut results: Vec<&Bitmap> = vec![];
        for token in tokenizer::analyze(text) {
            if let Some(ids) = self.0.get(&token) {
                results.push(ids);
            }
        }

        match results.is_empty() {
            true => Option::None,
            false => Option::from(Bitmap::fast_or(&results).iter().collect::<Vec<u32>>()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let mut index = Index::new();

        assert_eq!(Option::None, index.search("foo"));
        assert_eq!(Option::None, index.search("bar"));

        index.add(vec![
            Document {
                id: 1,
                text: "I bought a new GPU and sold my liver".to_string(),
                title: "a".to_string(),
                url: "b".to_string(),
            },
            Document {
                id: 2,
                text: "I have two healthy working livers".to_string(),
                title: "a".to_string(),
                url: "b".to_string(),
            },
        ]);

        assert_eq!(Option::None, index.search("a"));
        assert_vec!(vec![1, 2], index.search("liver").unwrap());
        assert_vec!(vec![2], index.search("works").unwrap());
        assert_vec!(vec![2], index.search("WoRk").unwrap());
        assert_vec!(vec![1], index.search("bought").unwrap());
        assert_eq!(Option::None, index.search("buy"));

        index.add(vec![Document {
            id: 3,
            text: "Drinking alcohol excessively damages livers permanently".to_string(),
            title: "a".to_string(),
            url: "b".to_string(),
        }]);
        assert_vec!(vec![1, 2, 3], index.search("liver").unwrap());

        index.add_token("ahihi".to_string(), 10000);
        assert_vec!(vec![10000], index.search("ahihi").unwrap());
    }
}
