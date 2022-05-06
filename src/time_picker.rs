use chrono::Local;
use chrono::prelude::*;
use cursive::align::HAlign;
use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{NamedView, ResizedView, SelectView};

use crate::ClockEntryColumn;
use crate::format;
use crate::granularity::Granularity;

pub fn new(col: ClockEntryColumn, value: Option<NaiveTime>, granularity: Granularity) -> NamedView<ResizedView<SelectView>> {
    let content = if value.is_some() {
        value.map(|it| format::format_naive_time(granularity, it)).expect("Time input entry should be some value")
    } else {
        now(granularity)
    };
    let entries = daily_clock_entries(granularity);

    let mut view = SelectView::new()
        .h_align(HAlign::Center)
        .popup()
        ;
    view.add_all_str(entries.iter());

    view
        .selected(entries.iter().position(|entry| entry.eq(content.as_str())).expect("Unable to find position for entry"))
        .fixed_width(15)
        .with_name(col.as_str())
}

pub fn get_time(s: &mut Cursive, col: ClockEntryColumn) -> NaiveTime {
    s.call_on_name(col.as_str(), |e: &mut ResizedView<SelectView>| {
        NaiveTime::parse_from_str(e.get_inner().selection().expect("Nothing selected in time field").as_str(), "%H:%M").expect("Unable to parse time from selection")
    }).expect(&format!("{} should be defined", col.as_str()))
}

fn now(granularity: Granularity) -> String {
    let now = Local::now();
    format::format_clock(granularity, now.hour(), now.minute(), now.second())
}

fn daily_clock_entries(granularity: Granularity) -> Vec<String> {
    let minute_step = match granularity {
        Granularity::Hour => 60,
        Granularity::Half => 30,
        Granularity::Quarter => 15,
        Granularity::FiveMinute => 5,
        Granularity::Minute => 1,
        Granularity::Second => 1,
    };
    let second_step = match granularity {
        Granularity::Second => 1,
        _ => 60,
    };
    (0..24).flat_map(|hour| {
        (0..60).step_by(minute_step).flat_map(|minute| {
            (0..60).step_by(second_step).map(move |second| {
                format::format_clock(granularity, hour, minute, second)
            }).collect::<Vec<String>>()
        }).collect::<Vec<String>>()
    }).collect()
}