use chrono::Utc;
use chrono_tz::Tz;

pub async fn format_log_message(message: &str, timezone: &Tz, is_error: bool) -> String {
    let time_now = Utc::now().with_timezone(timezone);
    let formatted_time = time_now.format("%Y-%m-%d %H:%M:%S");

    if !is_error {
        format!("[{}]: {}\n", formatted_time, message)
    } else {
        format!("!![{}]: {}!!\n", formatted_time, message)
    }
}
