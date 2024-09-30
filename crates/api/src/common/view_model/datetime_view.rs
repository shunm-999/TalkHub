use std::fmt::Display;

use chrono::{DateTime, SecondsFormat, TimeZone};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DateTimeViewModel {
    #[serde(rename = "dateTime")]
    pub date_time: String,
    pub timezone: String,
}

impl<Tz: TimeZone + Display> From<DateTime<Tz>> for DateTimeViewModel {
    fn from(date_time: DateTime<Tz>) -> Self {
        DateTimeViewModel {
            date_time: date_time.to_rfc3339_opts(SecondsFormat::Secs, true),
            timezone: date_time.timezone().to_string(),
        }
    }
}
