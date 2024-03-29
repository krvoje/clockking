use cursive::Cursive;
use cursive::direction::Orientation;
use cursive::traits::Nameable;
use cursive::views::{LinearLayout, TextView};
use cursive_table_view::TableView;

use crate::{app_context, CLOCK_ENTRIES_TABLE, ClockEntry, format, granularity_picker};
use crate::clock_entries_table::ClockEntryColumn;

pub const TOTAL_HOURS_CLOCKED: &str   = "Total clocked";
pub const TOTAL_HOURS_REMAINING: &str = "Left to clock";
pub const TOTAL_HOURS: &str           = "Total hours";
pub const RECORDING_STATUS: &str      = "RECORDING_STATUS";

pub fn new() -> LinearLayout {
    LinearLayout::new(Orientation::Vertical)
        .child(TextView::new(TOTAL_HOURS).with_name(TOTAL_HOURS))
        .child(TextView::new(TOTAL_HOURS_CLOCKED).with_name(TOTAL_HOURS_CLOCKED))
        .child(TextView::new(TOTAL_HOURS_REMAINING).with_name(TOTAL_HOURS_REMAINING))
        .child(TextView::new("No recording in progress.").with_name(RECORDING_STATUS))
}

pub fn update_stats(s: &mut Cursive) {
    let granularity = granularity_picker::get_granularity(s);
    let (total_seconds, total_seconds_clocked) = s.call_on_name(CLOCK_ENTRIES_TABLE, move |t: &mut TableView<ClockEntry, ClockEntryColumn>| {
        let items = t.borrow_items();
        (items.iter().map(|it| it.duration().num_seconds()).sum(),
         items.iter().filter(|it|it.is_clocked).map(|it| it.duration().num_seconds()).sum())
    }).unwrap();
    s.call_on_name(TOTAL_HOURS, move |t: &mut TextView| {
        t.set_content(format::format_hms_with_prompt(granularity, TOTAL_HOURS, total_seconds));
    });
    s.call_on_name(TOTAL_HOURS_CLOCKED, move |t: &mut TextView| {
        t.set_content(format::format_hms_with_prompt(granularity, TOTAL_HOURS_CLOCKED, total_seconds_clocked));
    });
    s.call_on_name(TOTAL_HOURS_REMAINING, move |t: &mut TextView| {
        t.set_content(format::format_hms_with_prompt(granularity, TOTAL_HOURS_REMAINING, total_seconds - total_seconds_clocked));
    });

    let context = app_context::fetch(s).ongoing_recording();
    let recording_status = if let Some(recording) = context {
        format!(
            "Recording '{}' ({} - ...)",
            recording.description,
            format::format_naive_time(granularity, recording.from)
        )
    } else {
        "No recording in progress.".to_string()
    };
    s.call_on_name(RECORDING_STATUS, move |t: &mut TextView| {
        t.set_content(recording_status);
    });
}