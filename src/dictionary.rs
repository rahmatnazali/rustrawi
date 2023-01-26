//! dictionary.rs
//!
//! Contains the implementation of word dictionary and its occurrences

use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::BufRead;

/// A dictionary structure to track word occurrences
pub struct Dictionary {
    words: HashMap<String, usize>,
}

impl Dictionary {
    /// Checks whether the dictionary contains the given word
    pub fn contains(&self, word: &str) -> bool {
        self.words.contains_key(word.to_lowercase().as_str())
    }

    /// Initialize empty Dictionary
    ///
    /// # Examples
    ///
    /// ```
    /// use rustrawi::dictionary::Dictionary;
    /// let dictionary = Dictionary::new();
    /// assert_eq!(dictionary.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            words: HashMap::new()
        }
    }

    /// Initialize the dictionary from a custom String collection
    ///
    /// # Examples
    ///
    /// ```
    /// use rustrawi::dictionary::Dictionary;
    /// let word_list = vec!["burung", "ayam", "kucing"];
    /// let mut dictionary = Dictionary::from_list(word_list);
    /// assert_eq!(dictionary.contains("burung"), true);
    /// assert_eq!(dictionary.contains("ayam"), true);
    /// assert_eq!(dictionary.contains("kucing"), true);
    /// ```
    pub fn from_list(word_list: Vec<&str>) -> Self {
        let mut dictionary = Dictionary::new();
        for word in word_list {
            dictionary.add(word.to_string())
        }
        dictionary
    }

    /// Read line of string from a given filename
    fn read_lines_from_file<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<std::path::Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    /// Initialize dictionary from a text file
    ///
    /// # Examples
    ///
    /// ```
    /// use rustrawi::dictionary::Dictionary;
    /// let dictionary = Dictionary::from_file("tests/example_word_list");
    /// assert_eq!(dictionary.len(), 3);
    /// ```
    pub fn from_file(filename: &str) -> Self {
        let mut dictionary = Dictionary::new();
        match Dictionary::read_lines_from_file(&filename) {
            Ok(lines) => {
                for line in lines {
                    if let Ok(value) = line {
                        dictionary.add(value);
                    }
                }
            },
            Err(e) => panic!("{}", e)
        }
        dictionary
    }

    /// Add a word to the dictionary (or update its occurrences)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustrawi::dictionary::Dictionary;
    /// let mut dictionary = Dictionary::new();
    /// dictionary.add(String::from("burung"));
    /// assert_eq!(dictionary.contains("burung"), true);
    /// ```
    pub fn add(&mut self, word: String) {
        let word = word.trim();
        if word == "" {
            return;
        }
        *self.words.entry(word.to_string()).or_insert(0_usize) += 1_usize;
    }

    /// Add list of word to the dictionary
    pub fn add_from_list(&mut self, words: Vec<&str>) {
        for word in words {
            self.add(word.to_string());
        }
    }

    /// Remove a word from dictionary
    /// Returns the word if it is removed successfully
    /// Returns None if the dictionary did not contains the word
    pub fn remove(&mut self, word: String) -> Option<String>{
        let word = word.trim();
        if word == "" {
            return None
        }
        match self.words.remove(word) {
            Some(_occurrence) => Some(word.to_string()),
            None => None
        }
    }

    /// Returns the length of the word dictionary
    pub fn len(&self) -> usize {
        self.words.len()
    }
}

#[cfg(test)]
mod dictionary_new_test {
    use super::*;

    #[test]
    fn should_generate_new_instance() {
        let dictionary = Dictionary::new();
        assert_eq!(dictionary.words.len(), 0);
    }
}

#[cfg(test)]
mod dictionary_from_test {
    use super::*;

    #[test]
    fn should_instantiate_with_custom_string_collection() {
        let word_list = vec!["burung", "ayam", "kucing"];
        let dictionary = Dictionary::from_list(word_list);
        assert_eq!(dictionary.contains("burung"), true);
    }

    #[test]
    fn should_instantiate_from_file() {
        let dictionary = Dictionary::from_file("tests/example_word_list");
        assert_eq!(dictionary.len(), 3);
        assert_eq!(dictionary.contains("burung"), true);
        assert_eq!(dictionary.contains("kucing"), true);
        assert_eq!(dictionary.contains("ayam"), true);
    }

    #[test]
    #[should_panic(expected = "No such file or directory (os error 2)")]
    fn should_panic_if_instantiating_from_invalid_file() {
        let _dictionary = Dictionary::from_file("tests/invalid_file");
    }
}

#[cfg(test)]
mod dictionary_contains_test {
    use super::*;

    #[test]
    fn should_return_false_if_string_is_not_contained() {
        let dictionary = Dictionary::new();
        assert_eq!(dictionary.contains(String::from("burung").as_str()), false);
    }

    #[test]
    fn should_return_true_if_string_is_contained() {
        let word_list = vec!["burung", "ayam", "kucing"];
        let dictionary = Dictionary::from_list(word_list);
        assert_eq!(dictionary.contains(String::from("burung").as_str()), true);
    }
}

#[cfg(test)]
mod dictionary_add_test {
    use super::*;

    #[test]
    fn should_be_able_to_add_word() {
        let mut dictionary = Dictionary::new();
        dictionary.add(String::from("burung"));
        assert_eq!(dictionary.contains(String::from("burung").as_str()), true);
        assert_eq!(dictionary.words.len(), 1);

        dictionary.add(String::from("kucing"));
        assert_eq!(dictionary.contains(String::from("kucing").as_str()), true);
        assert_eq!(dictionary.words.len(), 2);
    }

    #[test]
    fn should_not_add_empty_string() {
        let mut dictionary = Dictionary::new();
        dictionary.add(String::from(""));
        assert_eq!(dictionary.words.len(), 0);
    }

    #[test]
    fn should_not_add_whitespace() {
        let mut dictionary = Dictionary::new();
        dictionary.add(String::from(" "));
        assert_eq!(dictionary.words.len(), 0);
    }

    #[test]
    fn should_trim_before_add() {
        let mut dictionary = Dictionary::new();
        dictionary.add(String::from(" burung "));
        assert_eq!(dictionary.contains(String::from(" burung ").as_str()), false);
        assert_eq!(dictionary.contains(String::from("burung").as_str()), true);
        assert_eq!(dictionary.words.len(), 1);
    }

    #[test]
    fn should_add_from_list() {
        let mut dictionary = Dictionary::new();
        let word_list = vec!["burung", "kucing", "ayam"];
        dictionary.add_from_list(word_list);
        assert_eq!(dictionary.words.len(), 3);
        assert_eq!(dictionary.contains(String::from("burung").as_str()), true);
        assert_eq!(dictionary.contains(String::from("kucing").as_str()), true);
        assert_eq!(dictionary.contains(String::from("ayam").as_str()), true);
    }
}

#[cfg(test)]
mod dictionary_remove_test {
    use super::*;

    #[test]
    fn should_remove_entry() {
        let mut dictionary = Dictionary::new();
        dictionary.add(String::from("burung"));
        dictionary.add(String::from("kucing"));
        assert_eq!(dictionary.words.len(), 2);
        let result: Option<String> = dictionary.remove(String::from("burung"));
        assert_eq!(result.is_some(), true);
        assert_eq!(dictionary.words.len(), 1);
        assert_eq!(dictionary.contains(String::from("burung").as_str()), false);

        // try to re-delete the same word
        let result: Option<String> = dictionary.remove(String::from("burung"));
        assert_eq!(result.is_some(), false);
        assert_eq!(dictionary.words.len(), 1);
        assert_eq!(dictionary.contains(String::from("burung").as_str()), false);
    }

    #[test]
    fn should_return_none_on_empty_string() {
        let mut dictionary = Dictionary::new();
        dictionary.add(String::from("burung"));
        assert_eq!(dictionary.words.len(), 1);
        let result: Option<String> = dictionary.remove(String::from(""));
        assert_eq!(result.is_none(), true);
        assert_eq!(dictionary.words.len(), 1);
        assert_eq!(dictionary.contains(String::from("burung").as_str()), true);
    }

    #[test]
    fn should_return_none_on_whitespace() {
        let mut dictionary = Dictionary::new();
        dictionary.add(String::from("burung"));
        assert_eq!(dictionary.words.len(), 1);
        let result: Option<String> = dictionary.remove(String::from(" "));
        assert_eq!(result.is_none(), true);
        assert_eq!(dictionary.words.len(), 1);
        assert_eq!(dictionary.contains(String::from("burung").as_str()), true);
    }

    #[test]
    fn should_return_none_on_not_found() {
        let mut dictionary = Dictionary::new();
        dictionary.add(String::from("burung"));
        assert_eq!(dictionary.words.len(), 1);
        let result: Option<String> = dictionary.remove(String::from("kucing"));
        assert_eq!(result.is_none(), true);
    }
}

#[cfg(test)]
mod dictionary_len_test {
    use super::*;

    #[test]
    fn should_be_able_to_return_dictionary_length() {
        let mut dictionary = Dictionary::new();
        dictionary.add(String::from("burung"));
        assert_eq!(dictionary.len(), 1);

        dictionary.add(String::from("kucing"));
        assert_eq!(dictionary.len(), 2);
    }
}
