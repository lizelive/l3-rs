//! time is tricky
//! use the time lib over chrono because chrono was unmaintained for a while

use ::time as implementation;
pub use implementation::macros::{date, datetime, offset, time};

pub use implementation::{Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

pub type Timestamp = OffsetDateTime;

/// Returns the current time in UTC.
pub fn now() -> Timestamp {
    Timestamp::now_utc()
}