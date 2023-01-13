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
}