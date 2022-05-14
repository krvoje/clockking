use chrono::Local;
use chrono::prelude::*;
use cursive::align::HAlign;
use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{NamedView, ResizedView, SelectView};
use crate::clock_entries_table::ClockEntryColumn;

use crate::{format, granularity_picker};
use crate::granularity_picker::Granularity;

pub fn time_picker_input(col: ClockEntryColumn, value: Option<NaiveTime>, granularity: Granularity) -> NamedView<ResizedView<SelectView>> {
    let content = if value.is_some() {
        value.map(|it| format::format_naive_time(granularity, it)).expect("Time input entry should be some value")
    } else {
        now(granularity)
    };
    let entries = daily_clock_entries(granularity);

    let mut view = SelectView::new()
        .h_align(HAlign::Center)
        .popup()
        ;
    view.add_all_str(entries.iter());

    view
        .selected(entries.iter().position(|entry| entry.eq(content.as_str())).expect("Unable to find position for entry"))
        .fixed_width(15)
        .with_name(col.as_str())
}

pub fn time_picker_value(s: &mut Cursive, col: ClockEntryColumn) -> NaiveTime {
    let granularity = granularity_picker::get_granularity(s);
    s.call_on_name(col.as_str(), |e: &mut ResizedView<SelectView>| {
        parse_time(granularity, e.get_inner().selection().expect("Nothing selected in time field").as_str())
    }).expect(&format!("{} should be defined", col.as_str()))
}

pub fn parse_time(granularity: Granularity, value: &str) -> NaiveTime {
    let time = if granularity == Granularity::Scientific {
        NaiveTime::parse_from_str(value, "%H:%M:%S").expect("Unable to parse time from selection")
    } else {
        NaiveTime::parse_from_str(value, "%H:%M").expect("Unable to parse time from selection")
    }.with_nanosecond(0).unwrap();

    match granularity {
        Granularity::Relaxed => {time.with_minute(0).unwrap().with_second(0).unwrap()}
        Granularity::Reasonable => {time.with_minute(time.minute() / 30 * 30).unwrap().with_second(0).unwrap()}
        Granularity::Detailed => {time.with_minute(time.minute() / 15 * 15).unwrap().with_second(0).unwrap()}
        Granularity::Paranoid => {time.with_minute(time.minute() / 5 * 5).unwrap().with_second(0).unwrap()}
        Granularity::OCD => {time.with_second(0).unwrap()}
        Granularity::Scientific => time,
    }
}

pub fn now(granularity: Granularity) -> String {
    let now = Local::now();
    format::format_clock(granularity, now.hour(), now.minute(), now.second())
}

pub fn now_naive_time(granularity: Granularity) -> NaiveTime {
    let now_string = now(granularity);
    parse_time(granularity, now_string.as_str())
}

fn daily_clock_entries(granularity: Granularity) -> Vec<String> {
    let minute_step = match granularity {
        Granularity::Relaxed => 60,
        Granularity::Reasonable => 30,
        Granularity::Detailed => 15,
        Granularity::Paranoid => 5,
        Granularity::OCD => 1,
        Granularity::Scientific => 1,
    };
    let second_step = match granularity {
        Granularity::Scientific => 1,
        _ => 60,
    };
    (0..24).flat_map(|hour| {
        (0..60).step_by(minute_step).flat_map(|minute| {
            (0..60).step_by(second_step).map(move |second| {
                format::format_clock(granularity, hour, minute, second)
            }).collect::<Vec<String>>()
        }).collect::<Vec<String>>()
    }).collect()
}

#[cfg(test)]
mod parse_time_test {
    use chrono::NaiveTime;

    use crate::Granularity;
    use crate::time_picker::parse_time;

    #[test]
    fn parse_time_relaxed() {
        (0..24).for_each(|hour| {
            (0..60).for_each(move |minute| {
                assert_eq!(
                    parse_time(Granularity::Relaxed, format!("{:02$}:{:02$}", hour, minute, 2).as_str()),
                    NaiveTime::from_hms(hour, 0, 0)
                )
            })
        });
    }

    #[test]
    fn parse_time_reasonable() {
        (0..24).for_each(|hour| {
            (0..60).for_each(move |minute| {
                assert_eq!(
                    parse_time(Granularity::Reasonable, format!("{:02$}:{:02$}", hour, minute, 2).as_str()),
                    NaiveTime::from_hms(hour, minute / 30 * 30, 0)
                )
            })
        });
    }

    #[test]
    fn parse_time_detailed() {
        (0..24).for_each(|hour| {
            (0..60).for_each(move |minute| {
                assert_eq!(
                    parse_time(Granularity::Detailed, format!("{:02$}:{:02$}", hour, minute, 2).as_str()),
                    NaiveTime::from_hms(hour, minute / 15 * 15, 0)
                )
            })
        });
    }

    #[test]
    fn parse_time_paranoid() {
        (0..24).for_each(|hour| {
            (0..60).for_each(move |minute| {
                assert_eq!(
                    parse_time(Granularity::Paranoid, format!("{:02$}:{:02$}", hour, minute, 2).as_str()),
                    NaiveTime::from_hms(hour, minute / 5 * 5, 0)
                )
            })
        });
    }

    #[test]
    fn parse_time_ocd() {
        (0..24).for_each(|hour| {
            (0..60).for_each(move |minute| {
                assert_eq!(
                    parse_time(Granularity::OCD, format!("{:02$}:{:02$}", hour, minute, 2).as_str()),
                    NaiveTime::from_hms(hour, minute, 0)
                )
            })
        });
    }

    #[test]
    fn parse_time_scientific() {
        (0..24).for_each(|hour| {
            (0..60).for_each(move |minute| {
                (0..60).for_each(move |second| {
                    assert_eq!(
                        parse_time(Granularity::Scientific, format!("{:03$}:{:03$}:{:03$}", hour, minute, second, 2).as_str()),
                        NaiveTime::from_hms(hour, minute, second)
                    )
                })
            })
        });
    }
}

#[cfg(test)]
mod daily_clock_entries_test {
    use crate::Granularity;
    use crate::time_picker::daily_clock_entries;

    #[test]
    fn daily_clock_entries_relaxed() {
        assert_eq!(
            daily_clock_entries(Granularity::Relaxed),
            (0..24).map(|hour|{ format!("{:01$}:00", hour, 2) }).collect::<Vec<String>>()
        );
    }

    #[test]
    fn daily_clock_entries_reasonable() {
        assert_eq!(
            daily_clock_entries(Granularity::Reasonable),
            (0..24).flat_map(|hour|{
                [0, 30].iter().map(|minute|{
                    format!("{:02$}:{:02$}", hour, minute, 2)
                }).collect::<Vec<String>>()
            }).collect::<Vec<String>>()
        );
    }

    #[test]
    fn daily_clock_entries_detailed() {
        assert_eq!(
            daily_clock_entries(Granularity::Detailed),
            (0..24).flat_map(|hour|{
                [0, 15, 30, 45].iter().map(|minute|{
                    format!("{:02$}:{:02$}", hour, minute, 2)
                }).collect::<Vec<String>>()
            }).collect::<Vec<String>>()
        );
    }

    #[test]
    fn daily_clock_entries_paranoid() {
        assert_eq!(
            daily_clock_entries(Granularity::Paranoid),
            (0..24).flat_map(|hour|{
                [0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55].iter().map(|minute|{
                    format!("{:02$}:{:02$}", hour, minute, 2)
                }).collect::<Vec<String>>()
            }).collect::<Vec<String>>()
        );
    }

    #[test]
    fn daily_clock_entries_ocd() {
        assert_eq!(
            daily_clock_entries(Granularity::OCD),
            (0..24).flat_map(|hour|{
                (0..60).map(|minute|{
                    format!("{:02$}:{:02$}", hour, minute, 2)
                }).collect::<Vec<String>>()
            }).collect::<Vec<String>>()
        );
    }

    #[test]
    fn daily_clock_entries_scientific() {
        assert_eq!(
            daily_clock_entries(Granularity::Scientific),
            (0..24).flat_map(|hour| {
                (0..60).flat_map(|minute| {
                    (0..60).map(|second| {
                        format!("{:03$}:{:03$}:{:03$}", hour, minute, second, 2)
                    }).collect::<Vec<String>>()
                }).collect::<Vec<String>>()
            }).collect::<Vec<String>>()
        );
    }
}