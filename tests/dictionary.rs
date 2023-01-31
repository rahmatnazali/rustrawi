#[cfg(test)]
mod dictionary_test {
    use rustrawi::dictionary::Dictionary;

    #[test]
    fn should_initialize_dictionary_with_new() {
        let dictionary = Dictionary::new();
        assert_eq!(dictionary.len(), 0)
    }

    #[test]
    fn should_initialize_dictionary_with_custom_words() {
        let dictionary = Dictionary::from_list(vec!["burung", "kucing"]);
        assert_eq!(dictionary.len(), 2);
        assert_eq!(dictionary.contains("burung"), true);
        assert_eq!(dictionary.contains("kucing"), true);
        assert_eq!(dictionary.contains(""), false);
        assert_eq!(dictionary.contains(" "), false);
    }

    #[test]
    fn should_initalize_dictionary_from_file() {
        let dictionary = Dictionary::from_file("tests/example_word_list");
        assert_eq!(dictionary.len(), 3);
        assert_eq!(dictionary.contains("burung"), true);
        assert_eq!(dictionary.contains("kucing"), true);
        assert_eq!(dictionary.contains("ayam"), true);
        assert_eq!(dictionary.contains(""), false);
        assert_eq!(dictionary.contains(" "), false);
    }

    #[test]
    #[should_panic(expected="No such file or directory (os error 2)")]
    fn should_panic_with_invalid_file() {
        let dictionary = Dictionary::from_file("invalid_path");
    }

    #[test]
    fn should_add_word() {
        let mut dictionary = Dictionary::new();
        dictionary.add(String::from("burung"));
        assert_eq!(dictionary.contains("burung"), true);
    }

    #[test]
    fn should_add_word_in_normalized_manner() {
        let mut dictionary = Dictionary::new();
        dictionary.add(String::from(" Kucing "));
        assert_eq!(dictionary.contains("kucing"), true);
    }
}