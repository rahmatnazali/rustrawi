use crate::stemmer::context::Context;
use crate::stemmer::context::visitor::{Visitor, VisitorResult, VisitorType};

pub struct DontStemShortWord;

impl Visitor for DontStemShortWord {
    fn get_visitor_type(&self) -> VisitorType {
        VisitorType::GeneralVisitor
    }

    fn visit(&self, context: &Context) -> VisitorResult {
        if self.is_short_word(&(context.current_word)) {
            return VisitorResult::StopProcess
        }
        VisitorResult::None
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
    fn short_word_should_return_stop_process() {
        let dictionary = Dictionary::new();
        let mut context = Context::new("iya", &dictionary, None);

        let object = DontStemShortWord;
        let result = object.visit(&context);
        assert_eq!(result, VisitorResult::StopProcess);
    }

    #[test]
    fn long_word_should_return_none() {
        let dictionary = Dictionary::new();
        let mut context = Context::new("kambing", &dictionary, None);

        let object = DontStemShortWord;
        let result = object.visit(&context);
        assert_eq!(result, VisitorResult::None);
    }
}