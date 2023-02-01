use crate::stemmer::context::Context;
use crate::stemmer::context::visitor::{Visitor, VisitorType};

pub struct DontStemShortWord;

impl Visitor for DontStemShortWord {
    fn get_visitor_type(&self) -> VisitorType {
        VisitorType::GeneralVisitor
    }

    fn visit(&self, context: &mut Context) {
        if self.is_short_word(&context.current_word) {
            context.stop_process();
        };
    }
}

impl DontStemShortWord {
    fn is_short_word(&self, word: &str) -> bool {
        word.len() <= 3
    }
}

#[cfg(test)]
mod dont_stem_short_word_test {
    use crate::dictionary::Dictionary;
    use super::*;

    #[test]
    fn should_return_visitor_type() {
        let object = DontStemShortWord;
        assert_eq!(object.get_visitor_type(), VisitorType::GeneralVisitor);
    }

    #[test]
    fn should_aware_short_word() {
        let object = DontStemShortWord;
        assert_eq!(object.is_short_word(""), true);
        assert_eq!(object.is_short_word("a"), true);
        assert_eq!(object.is_short_word("ay"), true);
        assert_eq!(object.is_short_word("aya"), true);
    }

    #[test]
    fn should_aware_long_word() {
        let object = DontStemShortWord;
        assert_eq!(object.is_short_word("ayam"), false);
        assert_eq!(object.is_short_word("kucing"), false);
        assert_eq!(object.is_short_word("gajah"), false);
    }

    #[test]
    fn short_word_should_stop_process() {
        let dictionary = Dictionary::new();
        let mut context = Context::new("iya", &dictionary, None);
        assert_eq!(context.is_process_stopped, false);

        let object = DontStemShortWord;
        object.visit(&mut context);
        assert_eq!(context.is_process_stopped, true);
    }

    #[test]
    fn long_word_should_not_stop_process() {
        let dictionary = Dictionary::new();
        let mut context = Context::new("kambing", &dictionary, None);
        assert_eq!(context.is_process_stopped, false);

        let object = DontStemShortWord;
        object.visit(&mut context);
        assert_eq!(context.is_process_stopped, false);
    }
}