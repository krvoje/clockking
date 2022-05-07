use chrono::{NaiveTime, Timelike};
use cursive::{Cursive, traits::Nameable, views::{NamedView, SelectView}};
use cursive_table_view::TableView;
use serde::{Deserialize, Serialize};

use crate::{CLOCK_ENTRIES_TABLE, ClockEntryColumn, model::ClockEntry, update_stats};

const GRANULARITY: &str = "Granularity";

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Granularity {
    Relaxed,
    Reasonable,
    Detailed,
    Paranoid,
    OCD,
    Scientific,
}

pub fn new(selected_granularity: Granularity) -> NamedView<SelectView<Granularity>> {
    let mut view = SelectView::new().popup();
    view.add_item("Relaxed", Granularity::Relaxed);
    view.add_item("Reasonable", Granularity::Reasonable);
    view.add_item("Detailed", Granularity::Detailed);
    view.add_item("Paranoid", Granularity::Paranoid);
    view.add_item("OCD", Granularity::OCD);
    view.add_item("Scientific", Granularity::Scientific);
    view.set_selection(selected_granularity as usize);

    view.on_select(move |s, granularity| {
        select_granularity(s, granularity.clone());
    }).with_name(GRANULARITY)
}

fn select_granularity(s: &mut Cursive, granularity: Granularity) {
    s.call_on_name(CLOCK_ENTRIES_TABLE, |t: &mut TableView<ClockEntry, ClockEntryColumn>| {
        for item in t.borrow_items_mut() {
            normalize_for_granularity(item, granularity.clone());
        };
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
        Granularity::Relaxed => {
            it.with_minute(0).unwrap()
                .with_second(0).unwrap()
                .with_nanosecond(0).unwrap()
        },
        Granularity::Reasonable => {
            it.with_minute((it.minute() / 30) * 30).unwrap()
                .with_second(0).unwrap()
                .with_nanosecond(0).unwrap()
        },
        Granularity::Detailed => {
            it.with_minute((it.minute() / 15) * 15).unwrap()
                .with_second(0).unwrap()
                .with_nanosecond(0).unwrap()
        },
        Granularity::Paranoid => {
            it.with_minute((it.minute() / 5) * 5).unwrap()
                .with_second(0).unwrap()
                .with_nanosecond(0).unwrap()
        },
        Granularity::OCD => {
            it.with_second(0).unwrap()
                .with_nanosecond(0).unwrap()
        },
        Granularity::Scientific => {
            it.with_nanosecond(0).unwrap()
        },
    }
}