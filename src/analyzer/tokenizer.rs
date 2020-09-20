// use rust_stemmers::{Algorithm, Stemmer};
// use std::collections::HashSet;
use crate::analyzer::filters;
use std::str;

struct Span {
    start: i32,
    end: i32,
}

fn tokenize(text: &str) -> Vec<String> {
    let mut spans: Vec<Span> = Vec::with_capacity(32);
    let mut start = -1;
    for (i, character) in text.chars().enumerate() {
        if !character.is_alphanumeric() {
            if start >= 0 {
                spans.push(Span {
                    start: start,
                    end: i as i32,
                });
                start = -1;
            }
        } else {
            if start < 0 {
                start = i as i32;
            }
        }
    }

    if start >= 0 {
        spans.push(Span {
            start: start,
            end: text.len() as i32,
        })
    }
    let bytes = text.as_bytes();
    let mut strings: Vec<String> = Vec::with_capacity(spans.len());
    for elem in spans {
        let start = elem.start as usize;
        let end = elem.end as usize;
        if start >= end {
            continue;
        }

        strings.push(bytes[start..end].into_iter().map(|&b| b as char).collect())
    }
    strings
}

pub fn analyze(text: &str) -> Vec<String> {
    let mut tokens = tokenize(text);
    tokens = filters::to_lowercase(tokens);
    tokens = filters::drop_stop_words(tokens);
    tokens = filters::stem(tokens);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        struct TestCase<'a> {
            text: &'a str,
            tokens: Vec<String>,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                text: "",
                tokens: vec![],
            },
            TestCase {
                text: "new",
                tokens: vec_of_strings!["new"],
            },
            TestCase {
                text: "new - item",
                tokens: vec_of_strings!["new", "item"],
            },
            TestCase {
                text: "I bought a new GPU and sold my liver",
                tokens: vec_of_strings![
                    "I", "bought", "a", "new", "GPU", "and", "sold", "my", "liver"
                ],
            },
        ];

        for tc in cases {
            let output = tokenize(tc.text);
            assert_vec!(output, tc.tokens);
        }
    }

    #[test]
    fn test_analyze() {
        struct TestCase<'a> {
            text: &'a str,
            tokens: Vec<String>,
        }

        let cases: Vec<TestCase> = vec![
            TestCase {
                text: "",
                tokens: vec![],
            },
            TestCase {
                text: "new",
                tokens: vec_of_strings!["new"],
            },
            TestCase {
                text: "I bought a new GPU and sold my liver",
                tokens: vec_of_strings!["bought", "new", "gpu", "sold", "my", "liver"],
            },
        ];

        for tc in cases {
            let output = analyze(tc.text);
            assert_vec!(output, tc.tokens);
        }
    }
}
