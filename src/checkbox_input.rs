use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{Checkbox, NamedView};
use crate::ClockEntryColumn;

pub fn new(col: ClockEntryColumn, value: Option<bool>) -> NamedView<Checkbox> {
    Checkbox::new().with_checked(value == Some(true)).with_name(col.as_str())
}

pub fn get_value(s: &mut Cursive, input_name: &str) -> bool {
    s.call_on_name(input_name, |e: &mut Checkbox| {
        e.is_checked()
    }).expect(&format!("{} should be defined", input_name))
}