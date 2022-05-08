extern crate cursive_table_view;

use std::error::Error;

use cursive::{Cursive, CursiveExt};
use cursive::event::{Event, Key};

use granularity_picker::Granularity;

use crate::clock_entries_table::CLOCK_ENTRIES_TABLE;
use crate::model::*;

mod db;
mod model;
mod format;
mod text_area_input;
mod time_picker_input;
mod checkbox_input;
mod granularity_picker;
mod clock_entry_form;
mod autosave;
mod clock_entries_table;
mod stats_view;
mod main_dialog;

fn main() -> Result<(), Box<dyn Error>> {
    let mut siv = Cursive::default();

    let initial_clock_king = db::init_from_db(&mut siv);

    siv.add_layer(
        main_dialog::new(initial_clock_king)
    );

    siv.add_global_callback('q', main_dialog::quit);
    siv.add_global_callback(Event::CtrlChar('c'), main_dialog::quit);

    siv.add_global_callback(Key::Esc,strip_layer);

    stats_view::update_stats(&mut siv);

    siv.focus_name(CLOCK_ENTRIES_TABLE)?;
    autosave::start_autosave_loop(&siv);
    Ok(siv.run())
}

fn strip_layer(s: &mut Cursive) {
    s.pop_layer();
    if s.screen().is_empty() {
        main_dialog::quit(s);
    }
}