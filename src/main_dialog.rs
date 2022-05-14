use cursive::Cursive;
use cursive::direction::Orientation;
use cursive::event::Key;
use cursive::traits::{Nameable, Resizable};
use cursive::views::{Button, Dialog, DummyView, LinearLayout, OnEventView};

use crate::{clock_entries_table, ClockKing, db, granularity_picker, record, stats_view};

pub const RECORD_BUTTON: &str = "RECORD_BUTTON";

pub fn new(initial_clock_king: ClockKing) -> Dialog {
    Dialog::around(
        LinearLayout::new(Orientation::Vertical)
            .child(granularity_picker::new(initial_clock_king.granularity))
            .child(
                OnEventView::new(clock_entries_table::new(initial_clock_king))
                    .on_event(Key::Del, clock_entries_table::delete_current_entry)
                    .on_event('d', clock_entries_table::delete_current_entry)
                    .on_event('u', clock_entries_table::undo_delete)
                    .on_event(' ', clock_entries_table::mark_current_entry_as_clocked)
                    .on_event('a', clock_entries_table::add_new_entry)
                    .on_event('r', record::record)
            )
            .child(
                stats_view::new()
            )
            .child(
                LinearLayout::new(Orientation::Horizontal)
                    .child(Button::new("(A)dd", clock_entries_table::add_new_entry))
                    .child(DummyView.fixed_width(20))
                    .child(Button::new("Start (r)ecording", record::record).with_name(RECORD_BUTTON))
                    .child(DummyView.fixed_width(20))
                    .child(Button::new("(D)elete", clock_entries_table::delete_current_entry))
                    .child(DummyView.fixed_width(20))
                    .child(Button::new("(U)ndo Delete", clock_entries_table::undo_delete))
                    .child(DummyView.fixed_width(20))
                    .child(Button::new("(Q)uit", quit))
            )
    ).title("Clock King 👑")
}

pub fn quit(s: &mut Cursive) {
    db::save_to_db(s);
    s.quit();
}