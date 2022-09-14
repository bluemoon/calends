//! calends is a library for durations, intervals and other calendar related operations. It is
//! designed to work with chrono.
//!
//! # Durations of time
//!
//! A *RelativeDuration* is a unit of time that has some ability to be applied to a date to produce another
//! date.
//!
//! ```
//! use calends::RelativeDuration;
//! use chrono::NaiveDate;
//!
//! // This will allow you to add one month and then go back two days from the added month
//! let rd = RelativeDuration::months(1).with_days(-2);
//!
//! // It also compatible with NaiveDate
//! assert_eq!(
//!     NaiveDate::from_ymd(2022, 1, 1) + rd,
//!     NaiveDate::from_ymd(2022, 1, 30)
//! );
//! ```
//!
//! When applying durations to dates, it will apply in order if the largest units first e.g.
//! months will come before weeks. Therefore when you construct durations such as 1 month, -1 day
//! it will then move forward 1 month and then go backwards one day.
//!
//! ## Serialization
//!
//! There are two ways to serialize a RelativeDuration:
//! - The first one serializes it as an object.
//! - The second way is an ISO8601-2:2019 compatible serializer. Because the format is not
//! widely used yet we do not set it as the default (de)serializer.
//!
//! ```
//! use calends::RelativeDuration;
//! use calends::rd_iso8601;
//!
//! #[derive(Debug, serde::Deserialize, serde::Serialize)]
//! struct S {
//!    #[serde(
//!      deserialize_with = "rd_iso8601::deserialize",
//!      serialize_with = "rd_iso8601::serialize"
//!    )]
//!    rd: RelativeDuration,
//! }
//!
//! let rd = RelativeDuration::default().with_days(1).with_months(23).with_weeks(-1);
//! let s = S { rd };
//!
//! let rd_string = serde_json::to_string(&s).unwrap();
//! assert_eq!(rd_string, r#"{"rd":"P23M-1W1D"}"#);
//!
//! let parsed: S = serde_json::from_str(&rd_string).unwrap();
//! assert_eq!(rd, parsed.rd)
//! ```
//!
//! # Recurrence & Rules
//!
//! [Recurrence] allows you to specify a ruleset for how events (dates) repeat in time.
//!
//! ```
//! use calends::{Recurrence, Rule};
//! use chrono::NaiveDate;
//!
//! let date = NaiveDate::from_ymd(2022, 1, 1);
//! let end = NaiveDate::from_ymd(2022, 3, 1);
//!
//! let mut recur = Recurrence::with_start(Rule::monthly(), date).until(end);
//! assert_eq!(recur.next(), Some(NaiveDate::from_ymd(2022, 1, 1)));
//! assert_eq!(recur.next(), Some(NaiveDate::from_ymd(2022, 2, 1)));
//! assert_eq!(recur.next(), None);
//! ```
//!
//! # Intervals
//!
//! An interval is a span of time that can be bound or unbound. There are three important cases to
//! consider: closed, unbounded start, and unbounded end.
//!
//! - A closed interval has start and an end and can be repeated.
//! - An unbounded start interval does not have a start and only has an end.
//! - An unbounded end interval does not have an end but has a start.
//!
//! ## Closed interval
//!
//! In combination with RelativeDuration you can do things such as iterate the second to last day
//! of the month.
//!
//! ```
//! use calends::{Interval, RelativeDuration};
//! use chrono::NaiveDate;
//!
//! let duration = RelativeDuration::months(1).with_days(-2);
//! let start = NaiveDate::from_ymd(2022, 1, 1);
//!
//! let mut interval = Interval::closed_from_start(start, duration);
//! ```
//!
//! ## Serialization
//!
//! There are two ways to serialize a Interval:
//! - The first one serializes it as an object.
//! - The second way is an ISO8601-2:2019 compatible serializer. Because the format is not
//! widely used yet we do not set it as the default (de)serializer.
//!
//! ```
//! use chrono::NaiveDate;
//! use calends::{Interval, RelativeDuration, IntervalLike};
//! use calends::interval::marker::Start;
//!
//! #[derive(Debug, serde::Deserialize, serde::Serialize)]
//! struct S {
//!    i: Interval,
//! }
//!
//! let rd = RelativeDuration::default().with_days(1).with_months(23).with_weeks(-1);
//! let int = Interval::closed_from_start(NaiveDate::from_ymd(2022, 1, 1), rd);
//! let s = S { i: int.clone() };
//!
//! let int_string = serde_json::to_string(&s).unwrap();
//! assert_eq!(int_string, r#"{"i":"2022-01-01/2023-11-25"}"#);
//!
//! let parsed: S = serde_json::from_str(&int_string).unwrap();
//! assert_eq!(parsed.i.start_opt().unwrap(), int.start_opt().unwrap())
//! ```

pub mod duration;
pub mod grain;
pub mod interval;
mod parser;
pub mod recurrence;
pub mod unit;
pub mod util;

pub use crate::duration::serde::rd_iso8601;
pub use crate::duration::RelativeDuration;
pub use crate::interval::Interval;
pub use crate::recurrence::Rule;
pub use crate::unit::CalendarUnit;
pub use crate::util::*;
pub use crate::{interval::IntervalLike, recurrence::Recurrence};
