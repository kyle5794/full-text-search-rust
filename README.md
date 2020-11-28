# Simple Full-Text Search engine
Rust version of [Artem Krylysov](https://artem.krylysov.com/blog/2020/07/28/lets-build-a-full-text-search-engine/) tutorial to build a simple full-text search engine

Example:
Indexing documents from the abstract of English Wikipedia. The latest dump is available here https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-abstract1.xml.gz

Query for `Small wild cat` result:
- Original version: using built-in hashset as index
```
Loading documents...
Loading 614215 documents took: 105.616376785s
Indexing documents took: 84.009857986s
Query for: "Small wild cat"
Query took: 11.406094ms
Found 4742 results
```
- Using roaring bitmap as index
```
Loading documents...
Loading 617967 documents took: 107.236777035s
Indexing documents took: 72.404673792s
Query for: "Small wild cat"
Query took: 365.324µs
Found 4723 results
```
- Multi-thread indexing
    - 4 threads
    ```
    Loading documents...
    Loading 617967 documents took: 101.92519673s
    Spawning thread to index 154492 docs
    Spawning thread to index 154491 docs
    Spawning thread to index 154492 docs
    Spawning thread to index 154492 docs
    Indexing 617967 documents took: 18.047185589s
    Query for: "Small wild cat"
    Query took: 383.421µs
    Found 4723 results
    ```
    - 10 threads
    ```
    Loading documents...
    Loading 617967 documents: 105.858272009s
    Spawning thread to index 61797 docs
    Spawning thread to index 61797 docs
    Spawning thread to index 61797 docs
    Spawning thread to index 61796 docs
    Spawning thread to index 61797 docs
    Spawning thread to index 61797 docs
    Spawning thread to index 61796 docs
    Spawning thread to index 61796 docs
    Spawning thread to index 61797 docs
    Spawning thread to index 61797 docs
    Indexing 617967 documents: 12.031762159s
    Query for: "Small wild cat"
    Query took: 820.253µs
    Found 4723 results
    ```