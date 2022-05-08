use cursive::Cursive;
use cursive::traits::Nameable;
use cursive::views::{Dialog, ListView, NamedView};
use cursive_table_view::TableView;

use crate::{checkbox_input, CLOCK_ENTRIES_TABLE, ClockEntry, Granularity, stats_view, text_area_input, time_picker_input};
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
                    time_picker_input::new(ClockEntryColumn::From, entry.map(|it| it.from), granularity)
                )
                .child(
                    ClockEntryColumn::To.as_str(),
                    time_picker_input::new(ClockEntryColumn::To, entry.map(|it|it.to), granularity)
                )
                .child(
                    ClockEntryColumn::Description.as_str(),
                    text_area_input::new(ClockEntryColumn::Description, entry.map(|it| it.description.clone()))
                )
                .child(
                    ClockEntryColumn::IsClocked.as_str(),
                    checkbox_input::new(ClockEntryColumn::IsClocked, entry.map(|it| it.is_clocked))
                )
        )
        .button("Ok",move |s: &mut Cursive| submit_clock_entry(s, index)).with_name(CLOCK_ENTRY_FORM)
}

fn submit_clock_entry(s: &mut Cursive, index: Option<usize>) {
    let new_entry = ClockEntry {
        from: time_picker_input::get_time(s, ClockEntryColumn::From),
        to: time_picker_input::get_time(s, ClockEntryColumn::To),
        description: text_area_input::get_value(s, ClockEntryColumn::Description.as_str()),
        is_clocked: checkbox_input::get_value(s, ClockEntryColumn::IsClocked.as_str()) ,
    };
    s.call_on_name(CLOCK_ENTRIES_TABLE,   |table: &mut TableView<ClockEntry, ClockEntryColumn>| {
        index.map(|i| table.remove_item(i));
        table.insert_item(new_entry);
    }).expect("Unable to get clock entries table");
    stats_view::update_stats(s);
    s.pop_layer();
}