use std::collections::VecDeque;

use chrono::{Duration, NaiveTime};
use serde::{Deserialize, Serialize};

use crate::{Granularity, granularity_picker};

const UNDO_BUFFER_SIZE: usize = 20;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalContext {
    deleted: VecDeque<ClockEntry>,
    last_saved: ClockKing,
    recording: Option<ClockEntry>,
}

impl GlobalContext {
    pub(crate) fn new(model: &ClockKing) -> GlobalContext {
        GlobalContext {
            deleted: VecDeque::<ClockEntry>::default(),
            last_saved: model.clone(),
            recording: None,
        }
    }

    pub(crate) fn delete(&mut self, clock_entry: Option<ClockEntry>) {
        if let Some(it) = clock_entry {
            if self.deleted.len() >= UNDO_BUFFER_SIZE {
                self.deleted.pop_front();
            }
            self.deleted.push_back(it);
        }
    }

    pub(crate) fn undo(&mut self) -> Option<ClockEntry> {
        self.deleted.pop_back()
    }

    pub(crate) fn save(&mut self, clock_king: ClockKing) {
        self.last_saved = clock_king;
    }

    pub(crate) fn model_changed(&mut self, new_model: &ClockKing) -> bool {
        self.last_saved != new_model.clone()
    }

    pub(crate) fn start_recording(&mut self, new_entry: ClockEntry) {
        self.recording = Some(new_entry);
    }

    pub(crate) fn stop_recording(&mut self) -> ClockEntry {
        let result = self.recording.clone().expect("Recording should be in progress");
        self.recording = None;
        result
    }

    pub(crate) fn normalize_recording(&mut self, granularity: Granularity) {
        if self.is_recording() {
            let mut value = self.recording.clone().unwrap();
            granularity_picker::normalize_for_granularity(&mut value, granularity);
            self.recording = Some(value);
        }
    }

    pub(crate) fn is_recording(&self) -> bool {
        self.recording.is_some()
    }

    pub(crate) fn ongoing_recording(&self) -> Option<ClockEntry> {
        self.recording.clone()
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
    pub granularity: Granularity,
}

impl ClockEntry {
    pub fn duration(&self) -> Duration {
        self.to.signed_duration_since(self.from)
    }
}