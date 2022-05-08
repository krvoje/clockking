use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{NamedView, TextArea};

use crate::ClockEntryColumn;

pub fn new(col: ClockEntryColumn, value:Option<String>) -> NamedView<TextArea> {
    TextArea::new()
        .content(value.clone().get_or_insert(format!("")).to_string())
        .with_name(col.as_str())
}

pub fn get_value(s: &mut Cursive, input_name: &str) -> String {
    s.call_on_name(input_name, |e: &mut TextArea| {
        e.get_content().to_string()
    }).expect(&format!("{} should be defined", input_name))
}