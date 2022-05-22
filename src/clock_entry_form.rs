use cursive::Cursive;
use cursive::traits::Nameable;
use cursive::views::{Dialog, ListView, NamedView};

use crate::{ClockEntry, Granularity, input, time_picker};
use crate::clock_entries_table::ClockEntryColumn;

const CLOCK_ENTRY_FORM: &str = "edit_clock_entry";

pub fn new<F>(
    prompt: &str,
    entry: Option<&ClockEntry>,
    granularity: Granularity,
    on_submit: F,
) -> NamedView<Dialog>
where
    F: 'static + Fn(&mut Cursive) {
    Dialog::new()
        .title(prompt)
        .button("Cancel", |s| { s.pop_layer(); })
        .content(
            ListView::new()
                .child(
                    ClockEntryColumn::From.as_str(),
                    time_picker::time_picker_input(ClockEntryColumn::From, entry.map(|it| it.from), granularity)
                )
                .child(
                    ClockEntryColumn::To.as_str(),
                    time_picker::time_picker_input(ClockEntryColumn::To, entry.map(|it|it.to), granularity)
                )
                .child(
                    ClockEntryColumn::Client.as_str(),
                    input::text_area_input(ClockEntryColumn::Client, entry.map(|it| it.client.clone()))
                )
                .child(
                    ClockEntryColumn::Project.as_str(),
                    input::text_area_input(ClockEntryColumn::Project, entry.map(|it| it.project.clone()))
                )
                .child(
                    ClockEntryColumn::Description.as_str(),
                    input::text_area_input(ClockEntryColumn::Description, entry.map(|it| it.description.clone()))
                )
                .child(
                    ClockEntryColumn::IsClocked.as_str(),
                    input::checkbox_input(ClockEntryColumn::IsClocked, entry.map(|it| it.is_clocked))
                )
        )
        .button("Ok",on_submit).with_name(CLOCK_ENTRY_FORM)
}