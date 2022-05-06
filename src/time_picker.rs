use chrono::Local;
use chrono::prelude::*;
use cursive::align::HAlign;
use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{NamedView, ResizedView, SelectView};

use crate::{ClockEntryColumn, granularity};
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
    let granularity = granularity::get_granularity(s);
    s.call_on_name(col.as_str(), |e: &mut ResizedView<SelectView>| {
        parse_time(granularity, e.get_inner().selection().expect("Nothing selected in time field").as_str())
    }).expect(&format!("{} should be defined", col.as_str()))
}

pub fn parse_time(granularity: Granularity, value: &str) -> NaiveTime {
    if granularity == Granularity::Scientific {
        NaiveTime::parse_from_str(value, "%H:%M:%S").expect("Unable to parse time from selection")
    } else {
        NaiveTime::parse_from_str(value, "%H:%M").expect("Unable to parse time from selection")
    }
}

fn now(granularity: Granularity) -> String {
    let now = Local::now();
    format::format_clock(granularity, now.hour(), now.minute(), now.second())
}

fn daily_clock_entries(granularity: Granularity) -> Vec<String> {
    let minute_step = match granularity {
        Granularity::Relaxed => 60,
        Granularity::Reasonable => 30,
        Granularity::Detailed => 15,
        Granularity::Paranoid => 5,
        Granularity::OCD => 1,
        Granularity::Scientific => 1,
    };
    let second_step = match granularity {
        Granularity::Scientific => 1,
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