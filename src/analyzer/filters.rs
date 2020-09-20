use lazy_static::lazy_static;
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::HashSet;

lazy_static! {
    static ref STOP_WORDS: HashSet<String> = {
        [
            "a", "and", "be", "have", "i", "in", "of", "that", "the", "to",
        ]
        .iter()
        .map(|word| word.to_string())
        .collect()
    };
}

pub fn to_lowercase(tokens: Vec<String>) -> Vec<String> {
    tokens
        .into_iter()
        .map(|token| token.to_lowercase())
        .collect()
}

pub fn drop_stop_words(tokens: Vec<String>) -> Vec<String> {
    tokens
        .into_iter()
        .filter(|token| !STOP_WORDS.contains(token))
        .collect()
}

pub fn stem(tokens: Vec<String>) -> Vec<String> {
    let en_stemmer = Stemmer::create(Algorithm::English);
    tokens
        .into_iter()
        .map(|token| en_stemmer.stem(&token).to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_lowercase() {
        let input = vec_of_strings!["Haha", "PENCIL", "door", "ClOThes"];
        let expected = vec_of_strings!["haha", "pencil", "door", "clothes"];
        let output = to_lowercase(input);
        assert_vec!(expected, output);
    }

    #[test]
    fn test_drop_stop_words() {
        let input = vec_of_strings!["i", "bought", "a", "new", "gpu", "and", "sold", "my", "liver"];
        let expected = vec_of_strings!["bought", "new", "gpu", "sold", "my", "liver"];
        let output = drop_stop_words(input);
        assert_vec!(expected, output);
    }

    #[test]
    fn test_stem() {
        let input = vec_of_strings!["cat", "cats", "fish", "fishing", "fished", "airline"];
        let expected = vec_of_strings!["cat", "cat", "fish", "fish", "fish", "airlin"];
        let output = stem(input);
        assert_vec!(expected, output);
    }
}
