pub mod context;

use regex::Regex;
use crate::dictionary::Dictionary;
use crate::stemmer::context::Context;

pub struct Stemmer {
    dictionary: Dictionary,
    re_alphabet: Regex,
    re_whitespaces: Regex,
    re_is_plural: Regex,
}

impl Stemmer {
    /// Initialize Stemmer with default dictionary
    pub fn new() -> Self {
        Self {
            dictionary: Dictionary::from_file("src/data/kata-dasar.txt"),
            ..Self::empty()
        }
    }

    /// Initialize Stemmer with empty dictionary.
    ///
    /// Probably useful only for testing optimization
    pub fn empty() -> Self {
        Self {
            dictionary: Dictionary::new(),
            re_alphabet: Regex::new(r"[^a-z0-9 -]").unwrap(),
            re_whitespaces: Regex::new(r"( +)").unwrap(),
            re_is_plural: Regex::new(r"^(.*)-(ku|mu|nya|lah|kah|tah|pun)$").unwrap(),
        }
    }

    /// Returns stemmer dictionary length
    pub fn len(&self) -> usize {
        self.dictionary.len()
    }

    /// Evaluates whether the word is plural or not.
    ///
    /// `word` must be at a lowercase state.
    ///
    /// A word is evaluated as plural if it is strictly contains "-".
    /// For example: mobil-mobil, kapal-kapal, rumah-rumah.
    ///
    /// Extra steps must be taken if the word contains additional "-" suffix,
    /// as it also needs to be separated.
    /// For example: nikmat-nikmat-Nya.
    fn is_plural(&self, word: &str) -> bool {
        if self.re_is_plural.is_match(word) {
            let captures = self.re_is_plural.captures(word).unwrap();
            let first_group = captures.get(1).map_or("", |w| w.as_str());
            return first_group.contains("-");
        }
        return word.contains("-");
    }

    /// Removes symbols & characters other than alphabet
    ///
    /// Text is applied with two regex rule:
    /// - `r"[^a-z0-9 -]"`
    /// - `r"( +)"`
    fn normalize_text(&self, text: String) -> String {
        let lowercase_text = text.to_lowercase();
        let alphabet_only = self.re_alphabet.replace_all(&lowercase_text, " ");
        let single_whitespace_only = self.re_whitespaces.replace_all(&alphabet_only, " ");
        single_whitespace_only.trim().to_string()
    }

    /// Stem the given text.
    pub fn stem(&self, text: String) -> String {
        let normalized_text = self.normalize_text(text);
        let words = normalized_text.split(" ");

        let stemmed_words: Vec<String> = words.into_iter().map(|word| {
            if self.is_plural(word) {
                self.stem_plural_word(word)
            } else {
                self.stem_singular_word(word)
            }
        }).collect();
        stemmed_words.join(" ")
    }

    fn stem_singular_word(&self, word: &str) -> String {
        let mut context = Context::new(word, &self.dictionary, None);
        context.execute();
        context.get_resulting_word()
    }

    fn stem_plural_word(&self, word: &str) -> String {
        todo!()
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

#[cfg(test)]
mod is_plural_test {
    use super::*;

    #[test]
    fn should_return_false_on_singular() {
        let stemmer = Stemmer::new();
        assert_eq!(stemmer.is_plural("kucing"), false);
        assert_eq!(stemmer.is_plural("kucing-ku"), false);
    }

    #[test]
    fn should_return_true_on_plural() {
        let stemmer = Stemmer::new();
        assert_eq!(stemmer.is_plural("kucing-kucing"), true);
        assert_eq!(stemmer.is_plural("kucing - kucing"), true);
    }

    #[test]
    fn should_return_true_on_plural_with_suffix() {
        let stemmer = Stemmer::new();
        assert_eq!(stemmer.is_plural("kucing-kucing-nya"), true);
        assert_eq!(stemmer.is_plural("kucing - kucing-nya"), true);
        assert_eq!(stemmer.is_plural("kucing-kucing-ku"), true);
        assert_eq!(stemmer.is_plural("kucing - kucing-ku"), true);
    }
}

#[cfg(test)]
mod stemmer_test {
    use super::*;

    #[test]
    fn should_a() {
        let stemmer = Stemmer::new();
        let result = stemmer.stem(String::from("kucing, anjing, ayam"));
        println!("{}", result);
    }
}