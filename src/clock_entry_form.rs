use cursive::Cursive;
use cursive::traits::Nameable;
use cursive::views::{Checkbox, Dialog, ListView, NamedView, TextArea};
use cursive_table_view::TableView;

use crate::{CLOCK_ENTRIES_TABLE, ClockEntry, Granularity, stats_view, time_picker_input};
use crate::clock_entries_table::ClockEntryColumn;

const CLOCK_ENTRY_FORM: &str = "edit_clock_entry";

pub fn new(prompt: &str, entry: Option<&ClockEntry>, index: Option<usize>, granularity: Granularity) -> NamedView<Dialog> {
    Dialog::new()
        .title(prompt)
        .button("Cancel", |s| { s.pop_layer(); })
        .content(
            ListView::new()
                .child(
                    ClockEntryColumn::From.as_str(),
                    time_picker_input::time_picker(ClockEntryColumn::From, entry.map(|it| it.from), granularity)
                )
                .child(
                    ClockEntryColumn::To.as_str(),
                    time_picker_input::time_picker(ClockEntryColumn::To, entry.map(|it|it.to), granularity)
                )
                .child(
                    ClockEntryColumn::Description.as_str(),
                    text_area(ClockEntryColumn::Description, entry.map(|it| it.description.clone()))
                )
                .child(
                    ClockEntryColumn::IsClocked.as_str(),
                    checkbox_input(ClockEntryColumn::IsClocked, entry.map(|it| it.is_clocked))
                )
        )
        .button("Ok",move |s: &mut Cursive| submit_clock_entry(s, index)).with_name(CLOCK_ENTRY_FORM)
}

fn submit_clock_entry(s: &mut Cursive, index: Option<usize>) {
    let new_entry = ClockEntry {
        from: time_picker_input::time_picker_value(s, ClockEntryColumn::From),
        to: time_picker_input::time_picker_value(s, ClockEntryColumn::To),
        description: text_area_value(s, ClockEntryColumn::Description),
        is_clocked: checkbox_value(s, ClockEntryColumn::IsClocked) ,
    };
    s.call_on_name(CLOCK_ENTRIES_TABLE,   |table: &mut TableView<ClockEntry, ClockEntryColumn>| {
        index.map(|i| table.remove_item(i));
        table.insert_item(new_entry);
    }).expect("Unable to get clock entries table");
    stats_view::update_stats(s);
    s.pop_layer();
}

fn text_area(col: ClockEntryColumn, value:Option<String>) -> NamedView<TextArea> {
    TextArea::new()
        .content(value.clone().get_or_insert(format!("")).to_string())
        .with_name(col.as_str())
}

fn text_area_value(s: &mut Cursive, col: ClockEntryColumn) -> String {
    s.call_on_name(col.as_str(), |e: &mut TextArea| {
        e.get_content().to_string()
    }).expect(&format!("{} should be defined", col.as_str()))
}

fn checkbox_input(col: ClockEntryColumn, value: Option<bool>) -> NamedView<Checkbox> {
    Checkbox::new().with_checked(value == Some(true)).with_name(col.as_str())
}

fn checkbox_value(s: &mut Cursive, col: ClockEntryColumn) -> bool {
    s.call_on_name(col.as_str(), |e: &mut Checkbox| {
        e.is_checked()
    }).expect(&format!("{} should be defined", col.as_str()))
}