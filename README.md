# Simple Full-Text Search engine
Rust version of [Artem Krylysov](https://artem.krylysov.com/blog/2020/07/28/lets-build-a-full-text-search-engine/) tutorial to build a simple full-text search engine

Example:
Indexing documents from the abstract of English Wikipedia. The latest dump is available here https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-abstract1.xml.gz

Query for `Small wild cat` result:

```
Loading documents...
Loading 614215 documents took: 105.616376785s
Indexing documents took: 84.009857986s
Query for: \"Small wild cat\"
Query took: 11.406094ms
Found 4742 results
```