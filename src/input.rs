use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{Checkbox, NamedView, TextArea};

use crate::ClockEntryColumn;

pub fn text_area_input(col: ClockEntryColumn, value:Option<String>) -> NamedView<TextArea> {
    TextArea::new()
        .content(value.clone().get_or_insert(format!("")).to_string())
        .with_name(col.as_str())
}

pub fn checkbox_input(col: ClockEntryColumn, value: Option<bool>) -> NamedView<Checkbox> {
    Checkbox::new().with_checked(value == Some(true)).with_name(col.as_str())
}

pub fn get_text(s: &mut Cursive, input_name: &str) -> String {
    s.call_on_name(input_name, |e: &mut TextArea| {
        e.get_content().to_string()
    }).expect(&format!("{} should be defined", input_name))
}

pub fn get_bool(s: &mut Cursive, input_name: &str) -> bool {
    s.call_on_name(input_name, |e: &mut Checkbox| {
        e.is_checked()
    }).expect(&format!("{} should be defined", input_name))
}