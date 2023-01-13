use std::collections::HashMap;

struct Dictionary {
    words: HashMap<String, u64>,
}

impl Dictionary {
    /// Checks whether the dictionary contains the word `key`
    pub fn contains(&self, key: &str) -> bool {
        match self.words.get(key) {
            Some(number) => {
                &0_64 < number
            }
            None => false
        }
    }

    pub fn new() -> Self {
        Self {
            words: HashMap::new()
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
mod dictionary_contains_test {
    use super::*;

    #[test]
    fn should_return_false_if_string_is_not_contained() {
        let dictionary = Dictionary {
            words: HashMap::new()
        };
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