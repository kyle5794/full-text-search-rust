use full_text_search::document;
use full_text_search::search::index::Index;
use std::time::Instant;

fn main() {
    println!("Loading documents...");
    let start = Instant::now();
    let documents = match document::load_documents() {
        Ok(d) => d,
        Err(err) => panic!(err),
    };
    let duration = start.elapsed();
    println!("Loading {} documents took: {:?}", documents.len(), duration);

    let mut index = Index::new();
    let start = Instant::now();
    index.add(documents);
    let duration = start.elapsed();
    println!("Indexing documents took: {:?}", duration);

    println!("Query for: \"Small wild cat\"");
    let start = Instant::now();
    if let Some(result) = index.search("Small wild cat") {
        let duration = start.elapsed();
        println!("Query took: {:?}", duration);
        println!("Found {} results", result.len());
    } else {
        println!("oops");
    }
}
