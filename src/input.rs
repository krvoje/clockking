use cursive::Cursive;
use cursive::traits::Nameable;
use cursive::views::{Checkbox, NamedView, TextArea};
use crate::clock_entries_table::ClockEntryColumn;

pub fn text_area_input(col: ClockEntryColumn, value:Option<String>) -> NamedView<TextArea> {
    TextArea::new()
        .content(value.unwrap_or_default())
        .with_name(col.as_str())
}

pub fn text_area_value(s: &mut Cursive, col: ClockEntryColumn) -> String {
    s.call_on_name(col.as_str(), |e: &mut TextArea| {
        e.get_content().to_string()
    }).unwrap_or_else(|| panic!("{} should be defined", col.as_str()))
}

pub fn checkbox_input(col: ClockEntryColumn, value: Option<bool>) -> NamedView<Checkbox> {
    Checkbox::new().with_checked(value == Some(true)).with_name(col.as_str())
}

pub fn checkbox_value(s: &mut Cursive, col: ClockEntryColumn) -> bool {
    s.call_on_name(col.as_str(), |e: &mut Checkbox| {
        e.is_checked()
    }).unwrap_or_else(|| panic!("{} should be defined", col.as_str()))
}