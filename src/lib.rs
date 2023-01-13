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

    /// Add a word to the dictionary (or  update its occurrences)
    pub fn add(&mut self, word: String){
        *self.words.entry(word).or_insert(0_usize) += 1_usize;
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
}