extern crate cursive_table_view;

use std::cmp::Ordering;
use std::error::Error;

use cursive::{Cursive, CursiveExt};
use cursive::align::HAlign;
use cursive::direction::Orientation;
use cursive::event::Key;
use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, LinearLayout, ListView, NamedView, OnEventView, TextView};
use cursive_table_view::{TableView, TableViewItem};

use crate::model::*;

mod db;
mod model;
mod format;
mod input;
mod time_picker;

const CLOCK_ENTRIES_TABLE: &str   = "clock_entries";
const CLOCK_ENTRY_FORM: &str      = "edit_clock_entry";
const TOTAL_HOURS_CLOCKED: &str   = "Total clocked";
const TOTAL_HOURS_REMAINING: &str = "Left to clock";
const TOTAL_HOURS: &str           = "Total hours";

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ClockEntryColumn {
    From,
    To,
    Description,
    Duration,
    IsClocked,
}

impl ClockEntryColumn {
    fn as_str(&self) -> &str {
        match *self {
            ClockEntryColumn::From => "From",
            ClockEntryColumn::To => "To",
            ClockEntryColumn::Description => "Description",
            ClockEntryColumn::Duration => "Duration",
            ClockEntryColumn::IsClocked => "Clocked",
        }
    }
}

impl TableViewItem<ClockEntryColumn> for ClockEntry {
    fn to_column(&self, column: ClockEntryColumn) -> String {
        match column {
            ClockEntryColumn::From => self.from.format("%H:%M").to_string(),
            ClockEntryColumn::To => self.to.format("%H:%M").to_string(),
            ClockEntryColumn::Description => self.description.to_string(),
            ClockEntryColumn::Duration => format::duration_h_m(self.duration()),
            ClockEntryColumn::IsClocked => if self.is_clocked { "[x]".to_string() } else { "[ ]".to_string() },
        }
    }

    fn cmp(&self, other: &Self, column: ClockEntryColumn) -> Ordering where Self: Sized {
        match column {
            ClockEntryColumn::From => self.from.cmp(&other.from),
            ClockEntryColumn::To => self.to.cmp(&other.to),
            ClockEntryColumn::Description => self.description.cmp(&other.description),
            ClockEntryColumn::Duration => self.duration().cmp(&other.duration()),
            ClockEntryColumn::IsClocked => self.is_clocked.cmp(&other.is_clocked),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut siv = Cursive::default();
    let items = db::read_db();

    let mut table: TableView<ClockEntry, ClockEntryColumn> = TableView::<ClockEntry, ClockEntryColumn>::new()
        .column(ClockEntryColumn::From, ClockEntryColumn::From.as_str(), |c| {c.width_percent(10).align(HAlign::Center) })
        .column(ClockEntryColumn::To, ClockEntryColumn::To.as_str(), |c| {c.width_percent(10).align(HAlign::Center)})
        .column(ClockEntryColumn::Description, ClockEntryColumn::Description.as_str(), |c| {c.align(HAlign::Center)})
        .column(ClockEntryColumn::Duration, ClockEntryColumn::Duration.as_str(), |c| {c.width_percent(12).align(HAlign::Center)})
        .column(ClockEntryColumn::IsClocked, ClockEntryColumn::IsClocked.as_str(), |c| {c.width_percent(12).align(HAlign::Center)})
        .items(items)
        ;

    table.set_on_submit(move |s: &mut Cursive, _: usize, index: usize| {
        edit_entry(s,  index);
    });

    siv.add_layer(
        Dialog::around(
            LinearLayout::new(Orientation::Vertical)
                .child(
                    OnEventView::new(
                        table
                            .with_name(CLOCK_ENTRIES_TABLE)
                            .min_size((100,20))
                    ).on_event(Key::Del, |s| delete_current_entry(s))
                        .on_event('d', |s| delete_current_entry(s))
                        .on_event(' ',|s| mark_current_entry_as_clocked(s))
                        .on_event('a', |s| add_new_entry(s))
                )
                .child(
                    TextView::new(TOTAL_HOURS)
                        .with_name(TOTAL_HOURS)
                )
                .child(
                    TextView::new(TOTAL_HOURS_CLOCKED)
                        .with_name(TOTAL_HOURS_CLOCKED)
                )
                .child(
                    TextView::new(TOTAL_HOURS_REMAINING)
                        .with_name(TOTAL_HOURS_REMAINING)
                )
                .child(
                    LinearLayout::new(Orientation::Horizontal)
                        .child(Button::new("(A)dd", |s| add_new_entry(s)))
                        .child(DummyView.fixed_width(50))
                        .child(Button::new("(D)elete", |s| delete_current_entry(s)))
                        .child(DummyView.fixed_width(50))
                        .child(Button::new("(Q)uit", |s| s.quit()))
                )
        ).title("Clock King 👑")
    );

    siv.add_global_callback('q', Cursive::quit);

    siv.add_global_callback(Key::Esc,|s| {
        s.pop_layer();
        if s.screen().is_empty() {
            s.quit()
        }
    });

    update_stats(&mut siv);

    Ok(siv.run())
}

fn add_new_entry(s: &mut Cursive) {
    s.add_layer(edit_entry_form(None, 0));
}

fn delete_current_entry(s: &mut Cursive) {
    s.add_layer(
        cursive_extras::confirm_dialog(
            "Delete entry",
            "Are you sure?",
            |s| {
                s.pop_layer();
                s.call_on_name(CLOCK_ENTRIES_TABLE, move |t: &mut TableView<ClockEntry, ClockEntryColumn>| {
                    t.item().map(|index| t.remove_item(index));
                    let items = t.borrow_items();
                    db::save_to_db(items);
                }).unwrap();
                update_stats(s)
            }
        ));
}

fn update_stats(s: &mut Cursive) {
    let (total_minutes, total_minutes_clocked) = s.call_on_name(CLOCK_ENTRIES_TABLE, move |t: &mut TableView<ClockEntry, ClockEntryColumn>| {
        let items = t.borrow_items();
        (items.iter().map(|it| it.duration().num_minutes()).sum(),
         items.iter().filter(|it|it.is_clocked).map(|it| it.duration().num_minutes()).sum())
    }).unwrap();
    s.call_on_name(TOTAL_HOURS, move |t: &mut TextView| {
        t.set_content(format::h_m(TOTAL_HOURS, total_minutes));
    });
    s.call_on_name(TOTAL_HOURS_CLOCKED, move |t: &mut TextView| {
        t.set_content(format::h_m(TOTAL_HOURS_CLOCKED, total_minutes_clocked));
    });
    s.call_on_name(TOTAL_HOURS_REMAINING, move |t: &mut TextView| {
        t.set_content(format::h_m(TOTAL_HOURS_REMAINING, total_minutes - total_minutes_clocked));
    });
}

fn mark_current_entry_as_clocked(s: &mut Cursive) {
    s.call_on_name(CLOCK_ENTRIES_TABLE, move |t: &mut TableView<ClockEntry, ClockEntryColumn>| {
        t.item().map(|index| {
            let mut item = t.borrow_item_mut(index).expect("No entry at current index");
            item.is_clocked = !item.is_clocked;
        });
        let items = t.borrow_items();
        db::save_to_db(items);
    }).unwrap();
    update_stats(s);
}

fn edit_entry(s: &mut Cursive, index: usize) {
    let current_entry = s.call_on_name(CLOCK_ENTRIES_TABLE, move |t: &mut TableView<ClockEntry, ClockEntryColumn>| {
        t.borrow_item(index).expect("Unable to borrow item for edit").clone()
    });
    s.add_layer(edit_entry_form(current_entry, index));
}

fn edit_entry_form(current_entry: Option<ClockEntry>, index: usize) -> NamedView<Dialog> {
    Dialog::new()
        .title("Edit Clock Entry ⏰")
        .button("Cancel", |s| { s.pop_layer(); })
        .content(
            ListView::new()
                .child(
                    ClockEntryColumn::From.as_str(),
                    time_picker::new(ClockEntryColumn::From, current_entry.clone().map(|it| it.from))
                )
                .child(
                    ClockEntryColumn::To.as_str(),
                    time_picker::new(ClockEntryColumn::To, current_entry.clone().map(|it|it.to))
                )
                .child(
                    ClockEntryColumn::Description.as_str(),
                    input::text_area_input(ClockEntryColumn::Description, current_entry.clone().map(|it| it.description))
                )
                .child(
                    ClockEntryColumn::IsClocked.as_str(),
                    input::checkbox_input(ClockEntryColumn::IsClocked, current_entry.clone().map(|it| it.is_clocked))
                )
        )
        .button("Ok", move |s| {
            let new_entry = ClockEntry {
                from: time_picker::get_time(s, ClockEntryColumn::From),
                to: time_picker::get_time(s, ClockEntryColumn::To),
                description: input::get_text(s, ClockEntryColumn::Description.as_str()),
                is_clocked: input::get_bool(s, ClockEntryColumn::IsClocked.as_str()) ,
            };
            s.call_on_name(CLOCK_ENTRIES_TABLE,   |table: &mut TableView<ClockEntry, ClockEntryColumn>| {
                if current_entry.is_some() {
                    table.remove_item(index);
                }
                table.insert_item(new_entry);
                let items = table.borrow_items();
                db::save_to_db(items);
            }).expect("Unable to get clock entries table");
            update_stats(s);
            s.pop_layer();
        }).with_name(CLOCK_ENTRY_FORM)
}