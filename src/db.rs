use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter};
use crate::ClockEntry;

const DB_LOCATION: &str = "./.clockking/db.json";

pub fn read_db() -> Vec<ClockEntry> {
    create_dir_all("./.clockking").expect("Unable to create the .clockking directory");
    let file = File::open(DB_LOCATION).or_else(|_| File::create(DB_LOCATION)).expect("Unable to create nor open a .clockking file");
    let reader = BufReader::new(file);
    let u: Vec<ClockEntry> = serde_json::from_reader(reader).unwrap_or_else(|_|Vec::<ClockEntry>::default());

    u
}

pub fn save_to_db(entries: &[ClockEntry]) {
    let file = File::create(DB_LOCATION).expect("Unable to open DB file");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, entries).expect("Saving to DB failed");
}