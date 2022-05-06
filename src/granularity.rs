use chrono::{NaiveTime, Timelike};
use cursive::{Cursive, traits::Nameable, views::{NamedView, SelectView}};
use cursive_table_view::TableView;

use crate::{CLOCK_ENTRIES_TABLE, ClockEntryColumn, db, model::ClockEntry, update_stats};

const GRANULARITY: &str = "Granularity";

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Granularity {
    Hour,
    Half,
    Quarter,
    FiveMinute,
    Minute,
    Second,
}

pub fn new() -> NamedView<SelectView<Granularity>> {
    let mut view = SelectView::new().popup();
    view.add_item("Relaxed", Granularity::Hour);
    view.add_item("Reasonable", Granularity::Half);
    view.add_item("Detailed", Granularity::Quarter);
    view.add_item("Paranoid", Granularity::FiveMinute);
    view.add_item("OCD", Granularity::Minute);
    view.add_item("Scientific", Granularity::Second);
    view.set_selection(2);

    view.on_select(move |s, granularity| {
        select_granularity(s, granularity.clone());
    }).with_name(GRANULARITY)
}

fn select_granularity(s: &mut Cursive, granularity: Granularity) {
    s.call_on_name(CLOCK_ENTRIES_TABLE, |t: &mut TableView<ClockEntry, ClockEntryColumn>| {
        for item in t.borrow_items_mut() {
            normalize_for_granularity(item, granularity.clone());
        };
        db::save_to_db(t.borrow_items_mut());
    }).expect("The Clock entries table should be defined");
    update_stats(s);
}

pub fn get_granularity(s: &mut Cursive) -> Granularity {
    s.call_on_name(GRANULARITY, |view: &mut SelectView<Granularity>|{
        *view.selection().expect("Something should be selected")
    }).expect("The Granularity select should be defined")
}

fn normalize_for_granularity(item: &mut ClockEntry, granularity: Granularity) {
    item.from = normalize(item.from, granularity);
    item.to = normalize(item.to, granularity);
}

fn normalize(it: NaiveTime, granularity: Granularity) -> NaiveTime {
    match granularity {
        Granularity::Hour => {
            it.with_minute(0).unwrap()
                .with_second(0).unwrap()
                .with_nanosecond(0).unwrap()
        },
        Granularity::Half => {
            it.with_minute((it.minute() / 30) * 30).unwrap()
                .with_second(0).unwrap()
                .with_nanosecond(0).unwrap()
        },
        Granularity::Quarter => {
            it.with_minute((it.minute() / 15) * 15).unwrap()
                .with_second(0).unwrap()
                .with_nanosecond(0).unwrap()
        },
        Granularity::FiveMinute => {
            it.with_minute((it.minute() / 5) * 5).unwrap()
                .with_second(0).unwrap()
                .with_nanosecond(0).unwrap()
        },
        Granularity::Minute => {
            it.with_second(0).unwrap()
                .with_nanosecond(0).unwrap()
        },
        Granularity::Second => {
            it.with_nanosecond(0).unwrap()
        },
    }
}