/// A `Date` is a timestamp in milliseconds since the Unix epoch, in UTC.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct Date(u64);

impl Date {
    /// Constructs a new `Date`, where `ms` is milliseconds
    /// since the Unix epoch, in UTC.
    pub fn new(ms: u64) -> Self {
        Date(ms)
    }

    /// Returns the number of milliseconds since the Unix epoch, in UTC.
    pub fn now() -> Self {
        Date(chrono::offset::Utc::now().timestamp_millis() as u64)
    }
}

// Date -> u64
impl From<Date> for u64 {
    fn from(date: Date) -> u64 {
        date.0
    }
}

// Date -> u128
impl From<Date> for u128 {
    fn from(date: Date) -> u128 {
        date.0 as u128
    }
}

// DateTime -> Date
impl<Tz: chrono::TimeZone> From<chrono::DateTime<Tz>> for Date {
    fn from(date: chrono::DateTime<Tz>) -> Self {
        Date::new(date.timestamp_millis() as u64)
    }
}

// Dare -> DateTime<Utc>
impl From<Date> for chrono::DateTime<chrono::Utc> {
    fn from(date: Date) -> Self {
        chrono::DateTime::<chrono::Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp(date.0 as i64, 0),
            chrono::Utc,
        )
    }
}
