use chrono::Duration;

pub fn h_m(prompt: &str, total_minutes: i64) -> String {
    format!("{}:\t{}", prompt, format_h_m(total_minutes))
}

pub fn duration_h_m(duration: Duration) -> String {
    format_h_m(duration.num_minutes())
}

fn format_h_m(total_minutes: i64) -> String {
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;
    format!("{}h {:02$}m", hours, minutes, 2)
}