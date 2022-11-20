use chrono::{NaiveTime, Timelike};

use crate::granularity_picker::Granularity;

pub fn format_hms_with_prompt(granularity: Granularity, prompt: &str, total_seconds: i64) -> String {
    format!("{}:\t{}", prompt, format_hms(granularity, total_seconds))
}

pub fn format_hms(granularity: Granularity, total_seconds: i64) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds / 60) % 60;
    let seconds = total_seconds % 60;

    match granularity {
        Granularity::Relaxed => format!("{}h", hours),
        Granularity::Reasonable => format!("{}h {:02$}m", hours, minutes / 30 * 30, 2),
        Granularity::Detailed => format!("{}h {:02$}m", hours, minutes / 15 * 15, 2),
        Granularity::Paranoid => format!("{}h {:02$}m", hours, minutes / 5 * 5, 2),
        Granularity::Ocd => format!("{}h {:02$}m", hours, minutes, 2),
        Granularity::Scientific => format!("{}h {:03$}m {:03$}s", hours, minutes, seconds, 2),
    }
}

pub fn format_naive_time(granularity: Granularity, it: NaiveTime) -> String {
    format_clock(granularity, it.hour(), it.minute(), it.second())
}

pub fn format_clock(granularity: Granularity, hours: u32, minutes: u32, seconds: u32) -> String {
    match granularity {
        Granularity::Relaxed => format!("{:01$}:00", hours, 2),
        Granularity::Reasonable => format!("{:02$}:{:02$}", hours, (minutes / 30) * 30, 2),
        Granularity::Detailed => format!("{:02$}:{:02$}", hours, (minutes / 15) * 15, 2),
        Granularity::Paranoid => format!("{:02$}:{:02$}", hours, (minutes / 5) * 5, 2),
        Granularity::Ocd => format!("{:02$}:{:02$}", hours, minutes, 2),
        Granularity::Scientific => format!("{:03$}:{:03$}:{:03$}", hours, minutes, seconds, 2),
    }
}

#[cfg(test)]
mod format_hms_test {
    use crate::format::format_hms;
    use crate::Granularity;

    #[test]
    fn format_hms_relaxed() {
        (0..24).for_each(|hour| {
            (0..60).for_each(|minute| {
                (0..60).for_each(|second| {
                    assert_eq!(
                        format_hms(Granularity::Relaxed, hour * 3600 + minute * 60 + second),
                        format!("{}h", hour)
                    )
                })
            })
        });
    }

    #[test]
    fn format_hms_reasonable() {
        (0..24).for_each(|hour| {
            (0..60).for_each(|minute| {
                (0..60).for_each(|second| {
                    assert_eq!(
                        format_hms(Granularity::Reasonable, hour * 3600 + minute * 60 + second),
                        format!("{}h {:02$}m", hour, minute / 30 * 30, 2)
                    )
                })
            })
        });
    }

    #[test]
    fn format_hms_detailed() {
        (0..24).for_each(|hour| {
            (0..60).for_each(|minute| {
                (0..60).for_each(|second| {
                    assert_eq!(
                        format_hms(Granularity::Detailed, hour * 3600 + minute * 60 + second),
                        format!("{}h {:02$}m", hour, minute / 15 * 15, 2)
                    )
                })
            })
        });
    }

    #[test]
    fn format_hms_paranoid() {
        (0..24).for_each(|hour| {
            (0..60).for_each(|minute| {
                (0..60).for_each(|second| {
                    assert_eq!(
                        format_hms(Granularity::Paranoid, hour * 3600 + minute * 60 + second),
                        format!("{}h {:02$}m", hour, minute / 5 * 5, 2)
                    )
                })
            })
        });
    }

    #[test]
    fn format_hms_ocd() {
        (0..24).for_each(|hour| {
            (0..60).for_each(|minute| {
                (0..60).for_each(|second| {
                    assert_eq!(
                        format_hms(Granularity::Ocd, hour * 3600 + minute * 60 + second),
                        format!("{}h {:02$}m", hour, minute, 2)
                    )
                })
            })
        });
    }

    #[test]
    fn format_hms_scientific() {
        (0..24).for_each(|hour| {
            (0..60).for_each(|minute| {
                (0..60).for_each(|second| {
                    assert_eq!(
                        format_hms(Granularity::Scientific, hour * 3600 + minute * 60 + second),
                        format!("{}h {:03$}m {:03$}s", hour, minute, second, 2)
                    )
                })
            })
        });
    }

}

#[cfg(test)]
mod format_clock_test {
    use crate::format::format_clock;
    use crate::Granularity;

    #[test]
    fn format_clock_test() {
        (0..24).for_each(|hour| {
            (0..60).for_each(|minute| {
                (0..60).for_each(|second| {
                    assert_eq!(
                        format_clock(Granularity::Relaxed, hour, minute, second),
                        format!("{:01$}:00", hour, 2)
                    );
                    assert_eq!(
                        format_clock(Granularity::Reasonable, hour, minute, second),
                        format!("{:02$}:{:02$}", hour, minute / 30 * 30, 2)
                    );
                    assert_eq!(
                        format_clock(Granularity::Detailed, hour, minute, second),
                        format!("{:02$}:{:02$}", hour, minute / 15 * 15, 2)
                    );
                    assert_eq!(
                        format_clock(Granularity::Paranoid, hour, minute, second),
                        format!("{:02$}:{:02$}", hour, minute / 5 * 5, 2)
                    );
                    assert_eq!(
                        format_clock(Granularity::Ocd, hour, minute, second),
                        format!("{:02$}:{:02$}", hour, minute, 2)
                    );
                    assert_eq!(
                        format_clock(Granularity::Scientific, hour, minute, second),
                        format!("{:03$}:{:03$}:{:03$}", hour, minute, second, 2)
                    );
                })
            })
        });
    }
}