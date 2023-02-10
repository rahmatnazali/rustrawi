use crate::dictionary::Dictionary;
use crate::stemmer::confix_stripping::precedence_adjustment::PrecedenceAdjustment;
// use crate::stemmer::context::removal::{Removal};
use crate::stemmer::context::visitor::{Visitor, VisitorConfiguration, VisitorResult};
use crate::stemmer::context::visitor::dont_stem_short_word::DontStemShortWord;

// pub mod removal;
pub mod visitor;

pub struct Context<'a> {
    original_word: &'a str,
    current_word: String,
    result_word: Option<String>,
    is_process_stopped: bool,
    // removal_list: Vec<Removal>,
    dictionary: &'a Dictionary,
    general_visitors: Vec<Box<dyn Visitor>>,
    prefix_visitors: Vec<Box<dyn Visitor>>,
    suffix_visitors: Vec<Box<dyn Visitor>>,
}

impl<'a> Context<'a> {
    pub fn new(original_word: &'a str, dictionary: &'a Dictionary, visitor_configuration: Option<VisitorConfiguration>) -> Self {
        let mut visitor_configuration = visitor_configuration.unwrap_or(VisitorConfiguration::default());
        Self {
            original_word,
            current_word: original_word.to_string(),
            result_word: None,
            is_process_stopped: false,
            dictionary,
            general_visitors: std::mem::replace(&mut visitor_configuration.general_visitors, Vec::new()),
            prefix_visitors: std::mem::replace(&mut visitor_configuration.prefix_visitors, Vec::new()),
            suffix_visitors: std::mem::replace(&mut visitor_configuration.suffix_visitors, Vec::new()),
            // removal_list: vec![],
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
        for visitor in &(self.general_visitors) {
            let visitor_result: VisitorResult = visitor.visit(&self);
            match visitor_result {
                VisitorResult::StopProcess => { self.is_process_stopped = true; }
                VisitorResult::DoNothing => {}
            }
            if !self.is_process_stopped {
                break;
            }
        }

        if self.dictionary.contains(&self.current_word) {
            return;
        }

        // Confix Stripping: trying to remove prefix before suffix if the specification is met
        let precedence_adjustment = PrecedenceAdjustment::new();
        if precedence_adjustment.is_satisfied_by(self.original_word) {
            todo!("step 4, 5");
            todo!("step 2, 3");
        }

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