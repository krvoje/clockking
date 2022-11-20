use chrono::{NaiveTime, Timelike};
use cursive::{Cursive, traits::Nameable, views::SelectView};
use cursive::direction::Orientation;
use cursive::traits::Resizable;
use cursive::views::{LinearLayout, NamedView, TextView};
use cursive_table_view::TableView;
use serde::{Deserialize, Serialize};

use crate::{app_context, clock_entries_table, model::ClockEntry, stats_view};
use crate::clock_entries_table::ClockEntryColumn;

const GRANULARITY: &str = "Granularity";

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Granularity {
    Relaxed,
    Reasonable,
    Detailed,
    Paranoid,
    Ocd,
    Scientific,
}

pub fn new(selected_granularity: Granularity) -> LinearLayout {
    LinearLayout::new(Orientation::Horizontal)
        .child(TextView::new("Time granularity:").min_width(20))
        .child(create_view(selected_granularity))
}

fn create_view(selected_granularity: Granularity) -> NamedView<SelectView<Granularity>> {
    let mut view = SelectView::new().popup();
    view.add_item("Relaxed (1h)", Granularity::Relaxed);
    view.add_item("Reasonable (30m)", Granularity::Reasonable);
    view.add_item("Detailed (15m)", Granularity::Detailed);
    view.add_item("Paranoid (5m)", Granularity::Paranoid);
    view.add_item("OCD (1m)", Granularity::Ocd);
    view.add_item("Scientific (1s)", Granularity::Scientific);
    view.set_selection(selected_granularity as usize);

    view.on_submit(move |s, granularity| {
        select_granularity(s, *granularity);
    }).with_name(GRANULARITY)
}

fn select_granularity(s: &mut Cursive, granularity: Granularity) {
    s.call_on_name(clock_entries_table::CLOCK_ENTRIES_TABLE, |t: &mut TableView<ClockEntry, ClockEntryColumn>| {
        for item in t.borrow_items_mut() {
            normalize_for_granularity(item, granularity);
        };
    }).expect("The Clock entries table should be defined");
    app_context::fetch(s).normalize_recording(granularity);
    stats_view::update_stats(s);
}

pub fn get_granularity(s: &mut Cursive) -> Granularity {
    s.call_on_name(GRANULARITY, |view: &mut SelectView<Granularity>|{
        *view.selection().expect("Something should be selected")
    }).expect("The Granularity select should be defined")
}

pub fn normalize_for_granularity(item: &mut ClockEntry, granularity: Granularity) {
    item.from = normalize(item.from, granularity);
    item.to = normalize(item.to, granularity);
    item.granularity = granularity;
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
        Granularity::Ocd => {
            it.with_second(0).unwrap()
                .with_nanosecond(0).unwrap()
        },
        Granularity::Scientific => {
            it.with_nanosecond(0).unwrap()
        },
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveTime;

    use crate::Granularity;
    use crate::granularity_picker::normalize;

    #[test]
    fn test_normalize() {
        (0..24).for_each(|hour| {
            (0..60).for_each(|minute| {
                (0..60).for_each(|second| {
                    assert_eq!(
                        normalize(NaiveTime::from_hms(hour, minute, second), Granularity::Relaxed),
                        NaiveTime::from_hms(hour, 0, 0)
                    );
                    assert_eq!(
                        normalize(NaiveTime::from_hms(hour, minute, second), Granularity::Reasonable),
                        NaiveTime::from_hms(hour, minute / 30 * 30, 0)
                    );
                    assert_eq!(
                        normalize(NaiveTime::from_hms(hour, minute, second), Granularity::Detailed),
                        NaiveTime::from_hms(hour, minute / 15 * 15, 0)
                    );
                    assert_eq!(
                        normalize(NaiveTime::from_hms(hour, minute, second), Granularity::Paranoid),
                        NaiveTime::from_hms(hour, minute / 5 * 5, 0)
                    );
                    assert_eq!(
                        normalize(NaiveTime::from_hms(hour, minute, second), Granularity::Ocd),
                        NaiveTime::from_hms(hour, minute, 0)
                    );
                    assert_eq!(
                        normalize(NaiveTime::from_hms(hour, minute, second), Granularity::Scientific),
                        NaiveTime::from_hms(hour, minute, second)
                    );
                })
            })
        });
    }
}