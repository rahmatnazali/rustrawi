use crate::dictionary::Dictionary;
use crate::stemmer::context::removal::{Removal};
use crate::stemmer::context::visitor::{Visitor, VisitorResult};
use crate::stemmer::context::visitor::dont_stem_short_word::DontStemShortWord;

pub mod removal;
pub mod visitor;

pub struct Context<'a> {
    original_word: &'a str,
    current_word: String,
    result_word: Option<String>,
    is_process_stopped: bool,
    removal_list: Vec<Removal>,
    dictionary: &'a Dictionary,
    visitor_list: Vec<Box<dyn Visitor>>,
}

impl<'a> Context<'a> {
    pub fn new(original_word: &'a str, dictionary: &'a Dictionary, visitor_list: Option<Vec<Box<dyn Visitor>>>) -> Self {
        let visitor_list = visitor_list.unwrap_or_else(|| vec![
            Box::new(DontStemShortWord {}),
            // todo: add other visitor here
        ]);
        Self {
            original_word,
            current_word: original_word.to_string(),
            result_word: None,
            is_process_stopped: false,
            dictionary,
            visitor_list,
            removal_list: vec![],
        }
    }

    /// Execute the stemming process.
    /// The result can then be retrieved with .get_result()
    pub fn execute(&mut self) {

        // step 1
        if self.dictionary.contains(&self.current_word) {
            return;
        }

        // Iterate each visitor and run its modifier
        for visitor in &(self.visitor_list) {
            let visitor_result: VisitorResult = visitor.visit(&self);
            match visitor_result {
                VisitorResult::StopProcess => { self.is_process_stopped = true; },
                VisitorResult::DoNothing => {}
            }
            if !self.is_process_stopped {
                break;
            }
        }

        if self.dictionary.contains(&self.current_word) {
            return;
        }

        // todo!()
        // todo: complete the logic here
    }

    /// Returns the resulting word from stemming process
    pub fn get_resulting_word(&self) -> String {
        if self.result_word.is_some() {
            let cloned_word = self.result_word.clone();
            cloned_word.unwrap()
        } else {
            panic!("Resulting word is being called before available")
        }
    }
}

#[cfg(test)]
mod context_test {
    use super::*;

    #[test]
    fn should_stop_process_on_stop_process_result() {
        let dictionary = Dictionary::new();
        let mut context = Context::new("iya", &dictionary, None);
        assert_eq!(context.is_process_stopped, false);
        context.execute();
        assert_eq!(context.is_process_stopped, true);
    }

    #[test]
    fn should_not_stop_process_on_none_result() {
        let dictionary = Dictionary::new();
        let mut context = Context::new("ayam", &dictionary, None);
        assert_eq!(context.is_process_stopped, false);
        context.execute();
        assert_eq!(context.is_process_stopped, false);
    }
}