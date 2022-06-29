/// A `Date` is a timestamp in milliseconds since the Unix epoch, in UTC.
///
/// The date is in milliseconds.
#[derive(
    Default,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
)]
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

impl std::ops::Add for Date {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Date(self.0 + other.0)
    }
}

impl std::ops::Sub for Date {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Date(self.0 - other.0)
    }
}

// u128 -> Date
impl From<u128> for Date {
    fn from(ms: u128) -> Self {
        Date(ms as u64)
    }
}

// u64 -> Date
impl From<u64> for Date {
    fn from(ms: u64) -> Self {
        Date(ms)
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

// Date -> DateTime<Utc>
impl From<Date> for chrono::DateTime<chrono::Utc> {
    fn from(date: Date) -> Self {
        chrono::DateTime::<chrono::Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp(date.0 as i64, 0),
            chrono::Utc,
        )
    }
}
