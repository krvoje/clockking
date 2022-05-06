use chrono::{NaiveTime, Timelike};

use crate::granularity::Granularity;

pub fn format_hms_with_prompt(granularity: Granularity, prompt: &str, total_minutes: i64) -> String {
    format!("{}:\t{}", prompt, format_hms(granularity, total_minutes))
}

pub fn format_hms(granularity: Granularity, total_seconds: i64) -> String {
    let hours = total_seconds / 3600;
    let minutes = total_seconds / 60 % 60;
    let seconds = total_seconds % 60;
    
    match granularity {
        Granularity::Relaxed => format!("{}h", hours),
        Granularity::Reasonable => format!("{}h {:02$}m", hours, minutes / 30 * 30, 2),
        Granularity::Detailed => format!("{}h {:02$}m", hours, minutes / 15 * 15, 2),
        Granularity::Paranoid => format!("{}h {:02$}m", hours, minutes / 5 * 5, 2),
        Granularity::OCD => format!("{}h {:02$}m", hours, minutes, 2),
        Granularity::Scientific => format!("{}h {:03$}m {:03$}s", hours, minutes, seconds, 2),
    }
}

pub fn format_clock(granularity: Granularity, hours: u32, minutes: u32, seconds: u32) -> String {
    match granularity {
        Granularity::Relaxed => format!("{}:00", hours),
        Granularity::Reasonable => format!("{:02$}:{:02$}", hours, (minutes / 30) * 30, 2),
        Granularity::Detailed => format!("{:02$}:{:02$}", hours, (minutes / 15) * 15, 2),
        Granularity::Paranoid => format!("{:02$}:{:02$}", hours, (minutes / 5) * 5, 2),
        Granularity::OCD => format!("{:02$}:{:02$}", hours, minutes, 2),
        Granularity::Scientific => format!("{:03$}:{:03$}:{:03$}", hours, minutes, seconds, 2),
    }
}

pub fn format_naive_time(granularity: Granularity, it: NaiveTime) -> String {
    format_clock(granularity, it.hour(), it.minute(), it.second())
}