use std::collections::VecDeque;
use chrono::{Duration, NaiveTime};
use serde::{Deserialize, Serialize};
use crate::Granularity;

const UNDO_BUFFER_SIZE: usize = 20;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalContext {
    pub deleted: VecDeque<ClockEntry>,
    pub last_saved: ClockKing,
}

impl GlobalContext {
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ClockKing {
    pub clock_entries: Vec<ClockEntry>,
    pub granularity: Granularity,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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