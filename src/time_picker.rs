use chrono::Local;
use chrono::prelude::*;
use cursive::align::HAlign;
use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{NamedView, ResizedView, SelectView};

use crate::ClockEntryColumn;

pub fn new(col: ClockEntryColumn, value: Option<NaiveTime>) -> NamedView<ResizedView<SelectView>> {
    let content = if value.is_some() {
        value.map(|it| it.format("%H:%M").to_string()).expect("Time input entry should be some value")
    } else {
        now()
    };
    let entries = daily_clock_entries();

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

fn granularity() -> u32 {
    15
}

fn now() -> String {
    let now = Local::now();
    let hour = now.hour();
    let minute = now.minute();
    format!(
        "{:02$}:{:02$}",
        hour,
        (minute / granularity()) * granularity(),
        2)
}

fn daily_clock_entries() -> Vec<String> {
    (0..24).flat_map(|hour| {
        (0..60).step_by(usize::try_from(granularity()).unwrap()).map(|minute| {
            format!("{:02$}:{:02$}", hour, minute, 2)
        }).collect::<Vec<String>>()
    }).collect()
}