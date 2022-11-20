use std::cmp::Ordering;
use std::ops::Add;

use chrono::Duration;
use cursive::align::HAlign;
use cursive::Cursive;
use cursive::traits::{Nameable, Resizable};
use cursive::views::{NamedView, ResizedView};
use cursive_table_view::{TableView, TableViewItem};

use crate::{app_context, clock_entry_form, ClockEntry, ClockKing, format, granularity_picker, input, stats_view, time_picker};

pub const CLOCK_ENTRIES_TABLE: &str   = "clock_entries";

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ClockEntryColumn {
    From,
    To,
    Description,
    Duration,
    IsClocked,
}

impl ClockEntryColumn {
    pub(crate) fn as_str(&self) -> &str {
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
            ClockEntryColumn::From => format::format_naive_time(self.granularity, self.from),
            ClockEntryColumn::To => format::format_naive_time(self.granularity, self.to),
            ClockEntryColumn::Description => self.description.to_string(),
            ClockEntryColumn::Duration => format::format_hms(self.granularity, self.duration().num_seconds()),
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

pub fn new(model: ClockKing) -> ResizedView<NamedView<TableView<ClockEntry, ClockEntryColumn>>> {
    let mut table: TableView<ClockEntry, ClockEntryColumn> = TableView::<ClockEntry, ClockEntryColumn>::new()
        .column(ClockEntryColumn::From, ClockEntryColumn::From.as_str(), |c| {c.width_percent(10).align(HAlign::Center) })
        .column(ClockEntryColumn::To, ClockEntryColumn::To.as_str(), |c| {c.width_percent(10).align(HAlign::Center)})
        .column(ClockEntryColumn::Description, ClockEntryColumn::Description.as_str(), |c| {c.align(HAlign::Center)})
        .column(ClockEntryColumn::Duration, ClockEntryColumn::Duration.as_str(), |c| {c.width_percent(12).align(HAlign::Center)})
        .column(ClockEntryColumn::IsClocked, ClockEntryColumn::IsClocked.as_str(), |c| {c.width_percent(12).align(HAlign::Center)})
        .items(model.clock_entries)
        ;


    table.set_on_submit(move |s: &mut Cursive, _: usize, index: usize| {
        edit_entry(s,  index);
    });

    table
        .with_name(CLOCK_ENTRIES_TABLE)
        .min_size((100,20))
}

fn edit_entry(s: &mut Cursive, index: usize) {
    let granularity = granularity_picker::get_granularity(s);
    let form = s.call_on_name(CLOCK_ENTRIES_TABLE, move |t: &mut TableView<ClockEntry, ClockEntryColumn>| {
        let current_entry = t.borrow_item(index).cloned();
        clock_entry_form::new(
            "Edit Clock Entry ⏰",
            current_entry.as_ref(),
            granularity,
            move |s: &mut Cursive| submit_clock_entry(s, Some(index)))
    }).unwrap();
    s.add_layer(form);
}

pub fn add_new_entry(s: &mut Cursive) {
    let granularity = granularity_picker::get_granularity(s);
    let template_entry: Option<ClockEntry> = s.call_on_name(CLOCK_ENTRIES_TABLE, move |t: &mut TableView<ClockEntry, ClockEntryColumn>| {
        t.item().and_then(|it| t.borrow_item(it).map(|it| ClockEntry {
            from: it.to,
            to: it.to.add(Duration::minutes(60)),
            description: String::from(""),
            is_clocked: false,
            granularity,
        }))
    }).unwrap();

    s.add_layer(
        clock_entry_form::new(
            "Add Clock Entry ⏰",
            template_entry.as_ref(),
            granularity,
            move |s: &mut Cursive| submit_clock_entry(s, None)
        )
    );
}

fn submit_clock_entry(s: &mut Cursive, index: Option<usize>) {
    let new_entry = ClockEntry {
        from: time_picker::time_picker_value(s, ClockEntryColumn::From),
        to: time_picker::time_picker_value(s, ClockEntryColumn::To),
        description: input::text_area_value(s, ClockEntryColumn::Description),
        is_clocked: input::checkbox_value(s, ClockEntryColumn::IsClocked),
        granularity: granularity_picker::get_granularity(s)
    };
    s.call_on_name(CLOCK_ENTRIES_TABLE,   |table: &mut TableView<ClockEntry, ClockEntryColumn>| {
        index.map(|i| table.remove_item(i));
        table.insert_item(new_entry);
    }).expect("Unable to get clock entries table");
    stats_view::update_stats(s);
    s.pop_layer();
}

pub fn delete_current_entry(s: &mut Cursive) {
    s.add_layer(
        cursive_extras::confirm_dialog(
            "Delete entry",
            "Are you sure?",
            |s| {
                s.pop_layer();
                let deleted = s.call_on_name(CLOCK_ENTRIES_TABLE, move |t: &mut TableView<ClockEntry, ClockEntryColumn>| {
                    t.item().and_then(|index| t.remove_item(index))
                }).unwrap();
                app_context::fetch(s).delete(deleted);
                stats_view::update_stats(s)
            }
        ));
}

pub fn undo_delete(s: &mut Cursive) {
    if let Some(deleted) = app_context::fetch(s).undo() { 
            s.call_on_name(CLOCK_ENTRIES_TABLE, move |t: &mut TableView<ClockEntry, ClockEntryColumn>| {
                t.insert_item(deleted);
            }); 
    }
    stats_view::update_stats(s);
}

pub fn mark_current_entry_as_clocked(s: &mut Cursive) {
    s.call_on_name(CLOCK_ENTRIES_TABLE, move |t: &mut TableView<ClockEntry, ClockEntryColumn>| {
        if let Some(index) = t.item() {
            let mut item = t.borrow_item_mut(index).expect("No entry at current index");
            item.is_clocked = !item.is_clocked;
        }
    }).unwrap();
    stats_view::update_stats(s);
}

pub fn get_clock_entries(s: &mut Cursive) -> Vec<ClockEntry> {
    s.call_on_name(CLOCK_ENTRIES_TABLE,   |table: &mut TableView<ClockEntry, ClockEntryColumn>| {
        table.borrow_items().to_vec()
    }).expect("Clock entries table not defined")
}