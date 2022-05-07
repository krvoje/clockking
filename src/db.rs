use std::collections::VecDeque;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter};

use cursive::Cursive;

use crate::{ClockEntry, ClockKing, GlobalContext, Granularity, granularity};

const DB_LOCATION: &str = "./.clockking/db.json";

pub fn init_from_db(s: &mut Cursive) -> ClockKing {
    create_dir_all("./.clockking").expect("Unable to create the .clockking directory");
    let file = File::open(DB_LOCATION).or_else(|_| File::create(DB_LOCATION)).expect("Unable to create nor open a .clockking file");
    let reader = BufReader::new(file);
    let u: ClockKing = serde_json::from_reader(reader).unwrap_or_else(|_| ClockKing {
        clock_entries: Vec::<ClockEntry>::default(),
        granularity: Granularity::Detailed,
    });
    s.set_user_data(GlobalContext {
        deleted: VecDeque::<ClockEntry>::default(),
        last_saved: u.clone(),
    });

    u
}

pub fn save_to_db(s: &mut Cursive) {
    let clock_entries = crate::get_clock_entries(s);
    let granularity = granularity::get_granularity(s);
    let new_model = ClockKing {
        clock_entries,
        granularity,
    };
    let last_saved = s.user_data::<GlobalContext>().map(|it| it.last_saved.clone()).expect("Global context should be defined");
    if last_saved != new_model {
        save_model_to_db(s, &new_model);
    }
}

fn save_model_to_db(s: &mut Cursive, clock_king: &ClockKing) {
    s.user_data::<GlobalContext>().map(|it| it.last_saved = clock_king.clone());
    let file = File::create(DB_LOCATION).expect("Unable to open DB file");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &clock_king).expect("Saving to DB failed");
}