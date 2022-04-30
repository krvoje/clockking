use chrono::{Duration, NaiveTime};
use serde::{Serialize, Deserialize};

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