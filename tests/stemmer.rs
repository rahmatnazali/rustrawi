#[cfg(test)]
mod stemmer_test {
    use rustrawi::stemmer::Stemmer;

    #[test]
    fn should_initialize_stemmer_with_default_dictionary() {
        let stemmer = Stemmer::new();
        assert_eq!(stemmer.len(), 29932);
    }
}