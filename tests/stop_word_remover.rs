#[cfg(test)]
mod stop_word_remover_test {
    use rustrawi::dictionary::Dictionary;
    use rustrawi::stop_word_remover::StopWordRemover;

    #[test]
    fn should_initialize_stop_word_remover_with_default_dictionary() {
        let stop_word_remover = StopWordRemover::new();
        let string = String::from("Kucing dan Ayam beigtu akrabnya hingga mereka tidak sadar bahwa mereka berbeda jenis");
        let removed = stop_word_remover.remove(string);
        assert_eq!(removed, "Kucing Ayam beigtu akrabnya hingga sadar berbeda jenis")
    }

    #[test]
    fn should_initialize_stop_word_remover_with_custom_dictionary() {
        let custom_dictionary = Dictionary::from_list(vec!["begitu"]);
        let stop_word_remover = StopWordRemover::from(custom_dictionary);

        let string = String::from("Kucing dan Ayam begitu akrab");
        let removed = stop_word_remover.remove(string);
        assert_eq!(removed, "Kucing dan Ayam akrab")
    }
}