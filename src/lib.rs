use std::collections::HashMap;

/// A dictionary structure to track word occurrences
struct Dictionary {
    words: HashMap<String, u64>,
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
    pub fn from(dictionary: HashMap<String, u64>) -> Self {
        Self {
            words: dictionary.clone()
        }
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
        let mut custom_dictionary: HashMap<String, u64> = HashMap::new();
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
        let mut words: HashMap<String, u64> = HashMap::new();
        words.insert(String::from("burung"), 1);

        let dictionary = Dictionary {
            words
        };
        assert_eq!(dictionary.contains(String::from("burung").as_str()), true);
    }
}