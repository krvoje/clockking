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
        Granularity::Hour => format!("{}h", hours),
        Granularity::Half => format!("{}h {:02$}m", hours, minutes / 30 * 30, 2),
        Granularity::Quarter => format!("{}h {:02$}m", hours, minutes / 15 * 15, 2),
        Granularity::FiveMinute => format!("{}h {:02$}m", hours, minutes / 5 * 5, 2),
        Granularity::Minute => format!("{}h {:02$}m", hours, minutes, 2),
        Granularity::Second => format!("{}h {:03$}m {:03$}s", hours, minutes, seconds, 2),
    }
}

pub fn format_clock(granularity: Granularity, hours: u32, minutes: u32, seconds: u32) -> String {
    match granularity {
        Granularity::Hour => format!("{}:00", hours),
        Granularity::Half => format!("{:02$}:{:02$}", hours, (minutes / 30) * 30, 2),
        Granularity::Quarter => format!("{:02$}:{:02$}", hours, (minutes / 15) * 15, 2),
        Granularity::FiveMinute => format!("{:02$}:{:02$}", hours, (minutes / 5) * 5, 2),
        Granularity::Minute => format!("{:02$}:{:02$}", hours, minutes, 2),
        Granularity::Second => format!("{:03$}:{:03$}:{:03$}", hours, minutes, seconds, 2),
    }
}

pub fn format_naive_time(granularity: Granularity, it: NaiveTime) -> String {
    format_clock(granularity, it.hour(), it.minute(), it.second())
}