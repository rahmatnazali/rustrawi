use crate::dictionary::Dictionary;

struct StopWordRemover {
    stop_word_dictionary: Dictionary,
}

impl StopWordRemover {

    /// Initialize StopWordRemover with given stop word dictionary.
    pub fn from(stop_word_dictionary: Dictionary) -> Self {
        Self {
            stop_word_dictionary
        }
    }

    /// Remove stop word.
    pub fn remove(&self, text: String) -> String {
        let filtered_words: Vec<_> = text.split_whitespace().into_iter().filter(
            |x| {
                !self.stop_word_dictionary.contains(x)
            }
        ).collect::<_>();
        let joined_string = filtered_words.join(" ");
        joined_string
    }
}

#[cfg(test)]
mod stop_word_remover_test {
    use super::*;

    #[test]
    fn should_remove_stop_word() {
        let stop_word_remover = StopWordRemover::from(
            Dictionary::from_list(
                vec!["dan", "atau"]
            )
        );
        let string = String::from("Kucing dan ayam");

        let clean_string = stop_word_remover.remove(string);
        assert_eq!(clean_string, "Kucing ayam");
    }
}
