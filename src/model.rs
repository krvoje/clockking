use std::collections::VecDeque;
use chrono::{Duration, NaiveTime};
use serde::{Deserialize, Serialize};

const UNDO_BUFFER_SIZE: usize = 20;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalContext {
    pub deleted: VecDeque<ClockEntry>,
}

impl GlobalContext {
    pub fn new() -> GlobalContext {
        GlobalContext {
            deleted: VecDeque::new(),
        }
    }

    pub fn delete(&mut self, clock_entry: Option<ClockEntry>) {
        clock_entry.map(|it| {
            if self.deleted.len() >= UNDO_BUFFER_SIZE {
                self.deleted.pop_front();
            }
            self.deleted.push_back(it);
        });
    }

    pub fn undo(&mut self) -> Option<ClockEntry> {
        self.deleted.pop_back()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClockEntry {
    pub from: NaiveTime,
    pub to: NaiveTime,
    pub description: String,
    pub is_clocked: bool,
}

impl ClockEntry {
    pub fn duration(&self) -> Duration {
        self.to.signed_duration_since(self.from)
    }
}