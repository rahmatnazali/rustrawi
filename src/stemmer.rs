use regex::Regex;
use crate::dictionary::Dictionary;

pub struct Stemmer {
    dictionary: Dictionary,
}

impl Stemmer {
    /// Initialize Stemmer with default dictionary
    pub fn new() -> Self {
        Self {
            dictionary: Dictionary::from_file("src/data/kata-dasar.txt")
        }
    }

    /// Initialize Stemmer with empty dictionary.
    ///
    /// Probably useful only for testing optimization
    pub fn empty() -> Self {
        Self {
            dictionary: Dictionary::new()
        }
    }

    /// Returns stemmer dictionary length
    pub fn len(&self) -> usize {
        self.dictionary.len()
    }

    /// Removes symbols & characters other than alphabet
    ///
    /// Text is applied with two regex rule:
    /// - `r"/[^a-z0-9 -]/im"`
    /// - `r"/( +)/im"`
    fn normalize_text(&self, text: String) -> String {
        let alphabet_regex = Regex::new(r"[^a-z0-9 -]").unwrap();
        let multiple_whitespace_regex = Regex::new(r"( +)").unwrap();

        let lowercase_text = text.to_lowercase();
        let alphabet_only_result = alphabet_regex.replace_all(&lowercase_text, " ");
        let multiple_whitespace_removed = multiple_whitespace_regex.replace_all(&alphabet_only_result, " ");

        multiple_whitespace_removed.trim().to_string()
    }
}

#[cfg(test)]
mod normalize_text_test {
    use super::*;

    #[test]
    fn should_lowercase_text() {
        let stemmer = Stemmer::empty();
        let string = String::from("Ayam Bebek Cacing");
        let normalized_text = stemmer.normalize_text(string);
        assert_eq!(normalized_text, "ayam bebek cacing")
    }

    #[test]
    fn should_remove_other_than_alphabet() {
        let stemmer = Stemmer::empty();
        let string = String::from("Ayam, Kambing, serta kucing; semuanya berbahagia.");
        let normalized_text = stemmer.normalize_text(string);
        assert_eq!(normalized_text, "ayam kambing serta kucing semuanya berbahagia")
    }

    #[test]
    fn should_remove_multiple_whitespace() {
        let stemmer = Stemmer::empty();
        let string = String::from("Ayam,  Kambing   , dan    Kucing   ");
        let normalized_text = stemmer.normalize_text(string);
        assert_eq!(normalized_text, "ayam kambing dan kucing")
    }
}