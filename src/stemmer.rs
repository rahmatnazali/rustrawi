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

    /// Returns stemmer dictionary length
    pub fn len(&self) -> usize {
        self.dictionary.len()
    }
}
