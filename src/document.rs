use quick_xml::de::from_reader;
use serde::Deserialize;
use std::fs;
use std::io::{BufReader, Result};

#[derive(Debug, Deserialize)]
pub struct Document {
    #[serde(skip)]
    pub id: i32,
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
    let file = fs::File::open("enwiki-latest-abstract1.xml")?;
    let reader = BufReader::new(file);
    let mut d: Docs = from_reader(reader).unwrap();

    let l = d.documents.len();
    for idx in 0..l {
        d.documents[idx].id = idx as i32;
    }
    Ok(d.documents)
}
