pub mod dont_stem_short_word;

use crate::stemmer::context::Context;

#[derive(PartialEq, Debug)]
pub enum VisitorType {
    PrefixVisitor,
    GeneralVisitor,
    SuffixVisitor,
}

#[derive(PartialEq, Debug)]
pub enum VisitorResult {
    StopProcess,
    None
}

pub trait Visitor {
    fn get_visitor_type(&self) -> VisitorType;
    fn visit(&self, context: &Context) -> VisitorResult;
}
