use crate::dictionary::Dictionary;
use crate::stemmer::context::removal::{Removal, RemovalTrait};
use crate::stemmer::context::visitor::{Visitor};

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
    // visitor_provider: VisitorProvider,
    // suffix_visitor_list: Vec<usize>,
    // prefix_visitor_list: Vec<usize>,
}

impl<'a> Context<'a> {
    pub fn new(original_word: &'a str, dictionary: &'a Dictionary, visitor_list: Option<Vec<Box<dyn Visitor>>>) -> Self {
        let visitor_list = visitor_list.unwrap_or_else(|| vec![
            todo!("add visitor list here according to the source code")
        ]);
        Self {
            original_word,
            current_word: original_word.to_string(),
            result_word: None,
            is_process_stopped: false,
            dictionary,

            visitor_list,
            // visitor_provider: VisitorProvider::new(),
            // suffix_visitor_list: vec![],
            // prefix_visitor_list: vec![],
            removal_list: vec![],
        }
    }

    /// Execute the stemming process.
    /// The result can then be retrieved with .get_result()
    pub fn execute(&self) {

        // step 1
        if self.dictionary.contains(&self.current_word) {
            return;
        }

        todo!()
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