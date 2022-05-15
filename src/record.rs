use cursive::views::Button;
use cursive_table_view::TableView;

use crate::{app_context, CLOCK_ENTRIES_TABLE, clock_entry_form, ClockEntry, Cursive, granularity_picker, input, stats_view, time_picker};
use crate::clock_entries_table::ClockEntryColumn;
use crate::main_dialog::RECORD_BUTTON;

pub fn record(s: &mut Cursive) {
    if app_context::fetch(s).is_recording() {
        stop_recording(s)
    } else {
        start_recording(s)
    }
}

fn start_recording(s: &mut Cursive) {
    let granularity = granularity_picker::get_granularity(s);
    let new_entry = ClockEntry {
        from: time_picker::now_naive_time(granularity),
        to: time_picker::now_naive_time(granularity),
        description: String::from(""),
        is_clocked: false,
    };
    s.add_layer(
        clock_entry_form::new(
            "Start recording",
            Some(&new_entry),
            granularity,
            submit_recording_entry
        )
    );
}

fn submit_recording_entry(s: &mut Cursive) {
    let new_entry = ClockEntry {
        from: time_picker::time_picker_value(s, ClockEntryColumn::From),
        to: time_picker::time_picker_value(s, ClockEntryColumn::To),
        description: input::text_area_value(s, ClockEntryColumn::Description),
        is_clocked: input::checkbox_value(s, ClockEntryColumn::IsClocked) ,
    };
    app_context::fetch(s).start_recording(new_entry);
    s.pop_layer();
    s.call_on_name(RECORD_BUTTON, |b: &mut Button |{
        b.set_label("Stop (r)ecording")
    });
    stats_view::update_stats(s);
}

fn stop_recording(s: &mut Cursive) {
    let granularity = granularity_picker::get_granularity(s);
    let mut new_entry = app_context::fetch(s).stop_recording();
    new_entry.to = time_picker::now_naive_time(granularity);
    s.add_layer(
        clock_entry_form::new(
            "Stop recording",
            Some(&new_entry),
            granularity,
            add_recording_entry
        )
    );
}

fn add_recording_entry(s: &mut Cursive) {
    let new_entry = ClockEntry {
        from: time_picker::time_picker_value(s, ClockEntryColumn::From),
        to: time_picker::time_picker_value(s, ClockEntryColumn::To),
        description: input::text_area_value(s, ClockEntryColumn::Description),
        is_clocked: input::checkbox_value(s, ClockEntryColumn::IsClocked) ,
    };
    s.call_on_name(CLOCK_ENTRIES_TABLE, |table: &mut TableView<ClockEntry, ClockEntryColumn>| {
        table.insert_item(new_entry);
    }).expect("Unable to get clock entries table");
    stats_view::update_stats(s);
    s.pop_layer();
    s.call_on_name(RECORD_BUTTON, |b: &mut Button |{
        b.set_label("Start (r)ecording")
    });
    stats_view::update_stats(s);
}