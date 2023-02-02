pub mod dont_stem_short_word;

use crate::stemmer::context::Context;
use crate::stemmer::context::visitor::dont_stem_short_word::DontStemShortWord;

#[derive(PartialEq, Debug)]
pub enum VisitorType {
    PrefixVisitor,
    GeneralVisitor,
    SuffixVisitor,
}

#[derive(PartialEq, Debug)]
pub enum VisitorResult {
    StopProcess,
    DoNothing,
}

pub trait Visitor {
    fn get_visitor_type(&self) -> VisitorType;
    fn visit(&self, context: &Context) -> VisitorResult;
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