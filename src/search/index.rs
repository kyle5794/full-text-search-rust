use crate::analyzer::tokenizer;
use crate::document::Document;
use std::collections::{HashMap, HashSet};

pub struct Index(pub HashMap<String, HashSet<i32>>);

impl Index {
    pub fn new() -> Index {
        Index(HashMap::new())
    }

    pub fn add(&mut self, documents: Vec<Document>) {
        for document in documents {
            for token in tokenizer::analyze(&document.text) {
                match self.0.get_mut(&token) {
                    Some(ids) => ids.insert(document.id),
                    None => {
                        let mut s = HashSet::new();
                        s.insert(document.id);
                        self.0.insert(token, s);
                        true
                    }
                };
            }
        }
    }

    pub fn search(&self, text: &str) -> Option<Vec<i32>> {
        let mut result: HashSet<i32> = HashSet::new();
        for token in tokenizer::analyze(text) {
            if let Some(ids) = self.0.get(&token) {
                result = result.union(&ids).cloned().collect();
            }
        }

        match result.is_empty() {
            true => Option::None,
            false => {
                let mut v32: Vec<i32> = result.iter().cloned().collect();
                v32.sort();
                Option::from(v32)
            }
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
    }
}
