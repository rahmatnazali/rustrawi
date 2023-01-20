use std::collections::HashMap;

/// A dictionary structure to track word occurrences
pub struct Dictionary {
    words: HashMap<String, usize>,
}

impl Dictionary {
    /// Checks whether the dictionary contains the word `key`
    pub fn contains(&self, word: &str) -> bool {
        self.words.contains_key(word)
    }

    /// Initialize empty Dictionary
    pub fn new() -> Self {
        Self {
            words: HashMap::new()
        }
    }

    /// Initialize Dictionary from a custom hashmap
    pub fn from(dictionary: HashMap<String, usize>) -> Self {
        Self {
            words: dictionary.clone()
        }
    }

    /// Initialize dictionary from a text file
    pub fn from_file(file_path: String, delimiter: Option<String>) {
        let delimiter = delimiter.unwrap_or(String::from("\n"));
        todo!("implement opening a file and load the list of words from it")
    }

    /// Add a word to the dictionary (or update its occurrences)
    pub fn add(&mut self, word: String) {
        let word = word.trim();
        if word == "" {
            return;
        }
        *self.words.entry(word.to_string()).or_insert(0_usize) += 1_usize;
    }

    /// Add list of word to the dictionary
    pub fn add_from_list(&mut self, words: Vec<String>) {
        for word in words {
            self.add(word);
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
    fn should_instantiate_with_custom_hashmap() {
        let mut custom_dictionary: HashMap<String, usize> = HashMap::new();
        custom_dictionary.insert(String::from("burung"), 1);

        let dictionary = Dictionary::from(custom_dictionary);
        assert_eq!(dictionary.contains("burung"), true);
    }

    #[test]
    fn should_instantiate_from_file() {
        let dictionary = Dictionary::from_file(String::from("file.txt"), None);
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
        let mut custom_dictionary: HashMap<String, usize> = HashMap::new();
        custom_dictionary.insert(String::from("burung"), 1);

        let dictionary = Dictionary::from(custom_dictionary);
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
        todo!("implement add_from_list")
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
