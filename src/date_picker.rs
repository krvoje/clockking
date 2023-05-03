use chrono::{ Date, Local, NaiveDate, NaiveTime, TimeZone, Utc};
use cursive::Cursive;
use cursive::direction::Orientation;
use cursive::traits::{Nameable, Resizable};
use cursive::views::{LinearLayout, NamedView, TextView};
use cursive_calendar_view::{CalendarView, EnglishLocale, ViewMode, WeekDay};
use cursive_table_view::TableView;
use crate::{app_context, clock_entries_table, stats_view};
use crate::clock_entries_table::ClockEntryColumn;
use crate::model::ClockEntry;

const DATE_PICKER: &str = "Date picker";

pub fn new(date: Option<NaiveDate>) -> LinearLayout {
    LinearLayout::new(Orientation::Horizontal)
        .child(TextView::new("Current date:").min_width(20))
        .child(create_view(date.unwrap_or(Utc::now().date_naive())))
}

fn create_view(selected_date: NaiveDate) -> NamedView<CalendarView<Utc, EnglishLocale>> {
    let mut calendar = CalendarView::<Utc, EnglishLocale>::new(Utc.from_utc_date(&selected_date));

    calendar.set_view_mode(ViewMode::Month);
    calendar.set_week_start(WeekDay::Monday);

    calendar.on_submit(move |s, date| {
        select_date(s, *date);
    }).with_name(DATE_PICKER)
}

fn select_date(s: &mut Cursive, date: Date<Utc>) {
    let entries = app_context::fetch(s).entries(date.naive_local());
    s.call_on_name(clock_entries_table::CLOCK_ENTRIES_TABLE, |t: &mut TableView<ClockEntry, ClockEntryColumn>| {
        t.set_items(entries);
    }).expect("The Clock entries table should be defined");
    stats_view::update_stats(s);
}