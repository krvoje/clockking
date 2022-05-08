use cursive::Cursive;
use cursive::direction::Orientation;
use cursive::traits::Nameable;
use cursive::views::{LinearLayout, TextView};
use cursive_table_view::TableView;

use crate::{CLOCK_ENTRIES_TABLE, ClockEntry, format, granularity_picker};
use crate::clock_entries_table::ClockEntryColumn;

pub const TOTAL_HOURS_CLOCKED: &str   = "Total clocked";
pub const TOTAL_HOURS_REMAINING: &str = "Left to clock";
pub const TOTAL_HOURS: &str           = "Total hours";

pub fn new() -> LinearLayout {
    LinearLayout::new(Orientation::Vertical)
        .child(TextView::new(TOTAL_HOURS).with_name(TOTAL_HOURS))
        .child(TextView::new(TOTAL_HOURS_CLOCKED).with_name(TOTAL_HOURS_CLOCKED))
        .child(TextView::new(TOTAL_HOURS_REMAINING).with_name(TOTAL_HOURS_REMAINING))
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
}