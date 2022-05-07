use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter};
use cursive::Cursive;

use crate::{ClockEntry, ClockKing, Granularity, granularity};

const DB_LOCATION: &str = "./.clockking/db.json";

pub fn read_db() -> ClockKing {
    create_dir_all("./.clockking").expect("Unable to create the .clockking directory");
    let file = File::open(DB_LOCATION).or_else(|_| File::create(DB_LOCATION)).expect("Unable to create nor open a .clockking file");
    let reader = BufReader::new(file);
    let u: ClockKing = serde_json::from_reader(reader).unwrap_or_else(|_|ClockKing {
        clock_entries: Vec::<ClockEntry>::default(),
        granularity: Granularity::Detailed,
    });

    u
}

pub fn save_to_db(s: &mut Cursive) {
    let clock_entries = crate::get_clock_entries(s);
    let granularity = granularity::get_granularity(s);
    save_model_to_db(
        ClockKing {
            clock_entries,
            granularity
        }
    );
}

fn save_model_to_db(clock_king: ClockKing) {
    let file = File::create(DB_LOCATION).expect("Unable to open DB file");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &clock_king).expect("Saving to DB failed");
}