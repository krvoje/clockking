extern crate cursive_table_view;

use std::cmp::Ordering;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter};

use chrono::Duration;
use chrono::prelude::*;
use cursive::{Cursive, CursiveExt};
use cursive::align::HAlign;
use cursive::direction::Orientation;
use cursive::event::Key;
use cursive::traits::*;
use cursive::views::{Button, Checkbox, Dialog, DummyView, LinearLayout, ListView, NamedView, ResizedView, SelectView, TextArea, TextView};
use cursive_table_view::{TableView, TableViewItem};
use serde::{Deserialize, Serialize};

const DB_LOCATION: &str = "./.clockking/db.json";
const CLOCK_ENTRIES_TABLE: &str = "clock_entries";
const CLOCK_ENTRY_FORM: &str = "edit_clock_entry";
const TOTAL_HOURS_CLOCKED: &str = "total_hours_clocked";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ClockEntry {
    from: NaiveTime,
    to: NaiveTime,
    description: String,
    is_clocked: bool,
}

impl ClockEntry {
    fn duration(&self) -> Duration {
        self.to.signed_duration_since(self.from)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum ClockEntryColumn {
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
            ClockEntryColumn::Duration => hours_minutes_string(self.duration()),
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

fn hours_minutes_string(duration: Duration) -> String {
    hours_minutes_from_total_minutes(duration.num_minutes())
}

fn hours_minutes_from_total_minutes(total_minutes: i64) -> String {
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;
    format!("{}h {}m", hours, minutes)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut siv = Cursive::default();
    let items = read_db();
    let total_minutes = items.iter().map(|it| it.duration().num_minutes()).sum();

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
                    table
                        .with_name(CLOCK_ENTRIES_TABLE)
                        .min_size((100,20))
                )
                .child(
                    TextView::new(total_hours_clocked(total_minutes))
                        .with_name(TOTAL_HOURS_CLOCKED)
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

    siv.add_global_callback('a', |s| add_new_entry(s));
    siv.add_global_callback('d',|s| delete_current_entry(s));
    siv.add_global_callback(Key::Del,|s| delete_current_entry(s));
    siv.add_global_callback(' ',|s| mark_current_entry_as_clocked(s));
    siv.add_global_callback('q', Cursive::quit);

    siv.add_global_callback(Key::Esc,|s| {
        s.pop_layer();
        if s.screen().is_empty() {
            s.quit()
        }
    });

    Ok(siv.run())
}

fn total_hours_clocked(total_minutes: i64) -> String {
    format!("Total clocked today: {}", hours_minutes_from_total_minutes(total_minutes))
}

fn add_new_entry(s: &mut Cursive) {
    s.add_layer(edit_entry_form(None, 0));
}

fn delete_current_entry(s: &mut Cursive) {
    let total_minutes = s.call_on_name(CLOCK_ENTRIES_TABLE, move |t: &mut TableView<ClockEntry, ClockEntryColumn>| {
        t.item().map(|index| t.remove_item(index));
        let items = t.borrow_items();
        save_to_db(items);
        items.iter().map(|it| it.duration().num_minutes()).sum()
    }).unwrap();
    s.call_on_name(TOTAL_HOURS_CLOCKED, move |t: &mut TextView| {
        t.set_content(total_hours_clocked(total_minutes));
    });
}

fn mark_current_entry_as_clocked(s: &mut Cursive) {
    s.call_on_name(CLOCK_ENTRIES_TABLE, move |t: &mut TableView<ClockEntry, ClockEntryColumn>| {
        t.item().map(|index| {
            let mut item = t.borrow_item_mut(index).expect("No entry at current index");
            item.is_clocked = !item.is_clocked;
        });
        let items = t.borrow_items();
        save_to_db(items);
    }).unwrap();
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
                .child(ClockEntryColumn::From.as_str(), time_input(ClockEntryColumn::From, current_entry.clone().map(|it| it.from)))
                .child(ClockEntryColumn::To.as_str(), time_input(ClockEntryColumn::To, current_entry.clone().map(|it|it.to)))
                .child(ClockEntryColumn::Description.as_str(), text_area_input(ClockEntryColumn::Description, current_entry.clone().map(|it| it.description)))
                .child(ClockEntryColumn::IsClocked.as_str(), checkbox_input(ClockEntryColumn::IsClocked, current_entry.clone().map(|it| it.is_clocked)))
        )
        .button("Ok", move |s| {
            let new_entry = ClockEntry {
                from: get_time(s, ClockEntryColumn::From.as_str()),
                to: get_time(s, ClockEntryColumn::To.as_str()),
                description: get_text(s, ClockEntryColumn::Description.as_str()),
                is_clocked: get_bool(s, ClockEntryColumn::IsClocked.as_str()) ,
            };
            let total_minutes = s.call_on_name(CLOCK_ENTRIES_TABLE,   |table: &mut TableView<ClockEntry, ClockEntryColumn>| {
                if current_entry.is_some() {
                    table.remove_item(index);
                }
                table.insert_item(new_entry);

                let items = table.borrow_items();
                save_to_db(items);
                items.iter().map(|it|it.duration().num_minutes()).sum()
            }).expect("Unable to get clock entries table");
            s.call_on_name(TOTAL_HOURS_CLOCKED, move |t: &mut TextView| {
                t.set_content(total_hours_clocked(total_minutes));
            });
            s.pop_layer();
        }).with_name(CLOCK_ENTRY_FORM)
}

fn time_input(col: ClockEntryColumn, value: Option<NaiveTime>) -> NamedView<ResizedView<SelectView>> {
    let content = if value.is_some() {
        value.map(|it| it.format("%H:%M").to_string()).expect("Time input entry should be some value")
    } else {
        eprintln!("{}", now());
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

fn text_area_input(col: ClockEntryColumn, value:Option<String>) -> NamedView<ResizedView<TextArea>> {
    TextArea::new()
        .content(value.clone().get_or_insert(format!("")).to_string())
        .fixed_width(15)
        .with_name(col.as_str())
}

fn checkbox_input(col: ClockEntryColumn, value: Option<bool>) -> NamedView<ResizedView<Checkbox>> {
    Checkbox::new().with_checked(value == Some(true)).fixed_width(15).with_name(col.as_str())
}

fn read_db() -> Vec<ClockEntry> {
    create_dir_all("./.clockking").expect("Unable to create the .clockking directory");
    let file = File::open(DB_LOCATION).or_else(|_| File::create(DB_LOCATION)).expect("Unable to create nor open a .clockking file");
    let reader = BufReader::new(file);
    let u: Vec<ClockEntry> = serde_json::from_reader(reader).unwrap_or_else(|_|Vec::<ClockEntry>::default());

    u
}

fn save_to_db(entries: &[ClockEntry]) {
    let file = File::create(DB_LOCATION).expect("Unable to open DB file");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, entries).expect("Saving to DB failed");
}

fn get_time(s: &mut Cursive, input_name: &str) -> NaiveTime {
    s.call_on_name(input_name, |e: &mut ResizedView<SelectView>| {
        NaiveTime::parse_from_str(e.get_inner().selection().expect("Nothing selected in time field").as_str(), "%H:%M").expect("Unable to parse time from selection")
    }).expect(&format!("{} should be defined", input_name))
}

fn get_text(s: &mut Cursive, input_name: &str) -> String {
    s.call_on_name(input_name, |e: &mut ResizedView<TextArea>| {
        e.get_inner().get_content().to_string()
    }).expect(&format!("{} should be defined", input_name))
}

fn get_bool(s: &mut Cursive, input_name: &str) -> bool {
    s.call_on_name(input_name, |e: &mut ResizedView<Checkbox>| {
        e.get_inner().is_checked()
    }).expect(&format!("{} should be defined", input_name))
}

fn now() -> String {
    let now = Local::now();
    let hour = now.hour();
    let minute = now.minute();
    let minute15 = if minute < 15 {
        0
    } else if minute < 30 {
        15
    } else if minute < 45 {
        30
    } else {
        45
    };
    format!("{:02$}:{:02$}", hour, minute15, 2)
}

fn daily_clock_entries() -> Vec<String> {
    (0..24).flat_map(|hour| {
        [0, 15, 30, 45].iter().map(|minute| {
            format!("{:02$}:{:02$}", hour, minute, 2)
        }).collect::<Vec<String>>()
    }).collect()
}