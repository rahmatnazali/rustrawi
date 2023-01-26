use crate::dictionary::Dictionary;

struct StopWordRemover {
    stop_word_dictionary: Dictionary,
}

impl StopWordRemover {
    /// Initialize StopWordRemover with default stop word dictionary.
    pub fn new() -> Self {
        Self {
            stop_word_dictionary: Dictionary::from_list(
                vec![
                    "yang", "untuk", "pada", "ke", "para", "namun", "menurut", "antara", "dia",
                    "dua", "ia", "seperti", "jika", "jika", "sehingga", "kembali", "dan", "tidak",
                    "ini", "karena", "kepada", "oleh", "saat", "harus", "sementara", "setelah",
                    "belum", "kami", "sekitar", "bagi", "serta", "di", "dari", "telah", "sebagai",
                    "masih", "hal", "ketika", "adalah", "itu", "dalam", "bisa", "bahwa", "atau",
                    "hanya", "kita", "dengan", "akan", "juga", "ada", "mereka", "sudah", "saya",
                    "terhadap", "secara", "agar", "lain", "anda", "begitu", "mengapa", "kenapa",
                    "yaitu", "yakni", "daripada", "itulah", "lagi", "maka", "tentang", "demi",
                    "dimana", "kemana", "pula", "sambil", "sebelum", "sesudah", "supaya", "guna",
                    "kah", "pun", "sampai", "sedangkan", "selagi", "sementara", "tetapi", "apakah",
                    "kecuali", "sebab", "selain", "seolah", "seraya", "seterusnya", "tanpa",
                    "agak", "boleh", "dapat", "dsb", "dst", "dll", "dahulu", "dulunya", "anu",
                    "demikian", "tapi", "ingin", "juga", "nggak", "mari", "nanti", "melainkan",
                    "oh", "ok", "seharusnya", "sebetulnya", "setiap", "setidaknya", "sesuatu",
                    "pasti", "saja", "toh", "ya", "walau", "tolong", "tentu", "amat", "apalagi",
                    "bagaimanapun",
                ]
            )
        }
    }

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
    fn default_stop_word_should_remove_stop_word() {
        let stop_word_remover = StopWordRemover::new();
        let string = String::from("Kucing dan ayam itu bermain seperti saudara");

        let clean_string = stop_word_remover.remove(string);
        assert_eq!(clean_string, "Kucing ayam bermain saudara");
    }

    #[test]
    fn custom_stop_word_should_remove_stop_word() {
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
