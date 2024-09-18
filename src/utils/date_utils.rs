use chrono::{DateTime, Utc};

/// Formats a DateTime into a string with a specific format.
pub fn format_date(date: DateTime<Utc>, format: &str) -> String {
    date.format(format).to_string()
}