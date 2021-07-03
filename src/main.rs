use std::sync::mpsc;
use std::time::Instant;
use full_text_search::index::Index;
use full_text_search::document;

fn main() {
    let index = create_index(10);
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

fn create_index(num_worker: usize) -> Index {
    println!("Loading documents...");
    let start = Instant::now();
    let documents = match document::load_documents_shard(num_worker) {
        Ok(d) => d,
        Err(err) => panic!(err),
    };
    let num_docs = documents.len();

    println!("Loading {} documents: {:?}", num_docs, start.elapsed());

    let mut index = Index::new();
    let (tx, rx) = mpsc::channel::<Vec<(String, u32)>>();

    let start = Instant::now();
    let mut workers = Vec::with_capacity(num_worker);
    for set in documents {
        let sender = mpsc::Sender::clone(&tx);
        let w = std::thread::spawn(move || {
            println!("Spawning thread to index {:?} docs", set.len());
            for elem in set {
                let tokens = document::parse_document(elem);
                sender.send(tokens).unwrap();
            }
        });

        workers.push(w);
    }

    drop(tx);
    for received in rx {
        for token in received {
            index.add_token(token.0, token.1)
        }
    }

    println!("Indexing {} documents: {:?}", num_docs, start.elapsed());

    index
}
