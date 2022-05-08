use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter};

use cursive::Cursive;

use crate::{clock_entries_table, ClockEntry, ClockKing, GlobalContext, Granularity, granularity_picker};

const DB_LOCATION: &str = "./.clockking/db.json";

pub fn init_from_db(s: &mut Cursive) -> ClockKing {
    create_dir_all("./.clockking").expect("Unable to create the .clockking directory");
    let file = File::open(DB_LOCATION).or_else(|_| File::create(DB_LOCATION)).expect("Unable to create nor open a .clockking file");
    let reader = BufReader::new(file);
    let u: ClockKing = serde_json::from_reader(reader).unwrap_or_else(|_| ClockKing {
        clock_entries: Vec::<ClockEntry>::default(),
        granularity: Granularity::Detailed,
    });
    s.set_user_data(GlobalContext::new(&u));

    u
}

pub fn save_to_db(s: &mut Cursive) {
    let clock_entries = clock_entries_table::get_clock_entries(s);
    let granularity = granularity_picker::get_granularity(s);
    let new_model = ClockKing {
        clock_entries,
        granularity,
    };
    let context = s.user_data::<GlobalContext>().expect("Global context should be defined");
    if context.model_changed(&new_model) {
        save_model_to_db(s, &new_model);
    }
}

fn save_model_to_db(s: &mut Cursive, clock_king: &ClockKing) {
    s.user_data::<GlobalContext>().map(|it| it.save(clock_king.clone()));
    let file = File::create(DB_LOCATION).expect("Unable to open DB file");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &clock_king).expect("Saving to DB failed");
}