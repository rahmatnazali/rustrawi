pub mod dont_stem_short_word;

use crate::stemmer::context::Context;
use crate::stemmer::context::visitor::dont_stem_short_word::DontStemShortWord;

#[derive(PartialEq, Debug)]
pub enum VisitorType {
    PrefixVisitor,
    GeneralVisitor,
    SuffixVisitor,
}

pub trait Visitor {
    fn get_visitor_type(&self) -> VisitorType;
    fn visit<'a>(&'a self, context: &'a Context) -> Option<VisitorResult>;
}

pub struct VisitorConfiguration {
    pub general_visitors: Vec<Box<dyn Visitor>>,
    pub prefix_visitors: Vec<Box<dyn Visitor>>,
    pub suffix_visitors: Vec<Box<dyn Visitor>>,
}

impl VisitorConfiguration {
    pub fn default() -> Self {
        Self {
            general_visitors: vec![
                Box::new(DontStemShortWord {}),
            ],
            prefix_visitors: vec![
                // todo: implement and add other visitor here
            ],
            suffix_visitors: vec![
                // todo: implement and add other visitor here
            ],
        }
    }
}

pub struct VisitorResult {
    pub original_word: String,
    pub current_word: String,
    pub result_word: Option<String>,
    pub should_process_stop: bool,
}

impl VisitorResult {
    /// Initialize VisitorResult object that only serve as stop_process flag
    pub fn stop_process(original_word: String, current_word: String, result_word: Option<String>) -> Self {
        Self {
            original_word: original_word.clone(),
            current_word: current_word.clone(),
            result_word: result_word.clone(),
            should_process_stop: true,
        }
    }
}