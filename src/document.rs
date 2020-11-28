use crate::analyzer::tokenizer;
use quick_xml::de::from_reader;
use serde::Deserialize;
use std::fs;
use std::io::{BufReader, Result};

#[derive(Debug, Deserialize)]
pub struct Document {
    #[serde(skip)]
    pub id: u32,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "abstract", default)]
    pub text: String,
    #[serde(rename = "url", default)]
    pub url: String,
}

#[derive(Debug, Deserialize)]
struct Docs {
    #[serde(rename = "doc", default)]
    pub documents: Vec<Document>,
}

pub fn load_documents() -> Result<Vec<Document>> {
    let file = fs::File::open("/home/kyle/learning/rust/data/wiki/enwiki-latest-abstract1.xml")?;
    let reader = BufReader::new(file);
    let mut d: Docs = from_reader(reader).unwrap();

    let l = d.documents.len();
    for idx in 0..l {
        d.documents[idx].id = idx as u32;
    }
    Ok(d.documents)
}

pub fn load_documents_shard(num_shards: usize) -> Result<Vec<Vec<Document>>> {
    let file = fs::File::open("/home/kyle/learning/rust/data/wiki/enwiki-latest-abstract1.xml")?;
    let reader = BufReader::new(file);
    let d: Docs = from_reader(reader).unwrap();
    let mut results: Vec<Vec<Document>> = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        results.push(vec![]);
    }
    let l = d.documents.len();
    for idx in 0..l {
        let i = idx % num_shards;
        results[i].push(Document {
            id: idx as u32,
            text: d.documents[idx].text.clone(),
            url: d.documents[idx].url.clone(),
            title: d.documents[idx].title.clone(),
        });
    }

    Ok(results)
}


pub fn parse_document(doc: Document) -> Vec<(String, u32)> {
    tokenizer::analyze(&doc.text)
        .iter()
        .cloned()
        .map(|token| (token, doc.id))
        .collect::<Vec<(String, u32)>>()
}
