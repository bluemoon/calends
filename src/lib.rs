//! # calends
//!
//! calends is a library for durations, intervals and other calendar related operations. It is
//! designed to work with chrono.
//!
//! # Durations of time
//!
//! A [RelativeDuration] is a unit of time that has some ability to be applied to a date to produce another
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
//! An interval is a span of time that can be bound or unbound. This means that you
//! can iterate until the beginning/end of the time. However in practice this will be limited by
//! chronos types.
//!
//! This will likely be used to do things like iterate by week, month, quarter, or year.
//!
//! ```
//! use calends::{Interval, IntervalLike, RelativeDuration};
//! use calends::interval::marker::{End, Start};
//! use chrono::NaiveDate;
//!
//! let start = NaiveDate::from_ymd(2022, 1, 1);
//! let duration = RelativeDuration::months(1);
//!
//! let mut interval = Interval::from_start(start, duration);
//!
//! assert_eq!(interval.start(), start);
//! assert_eq!(interval.end(), NaiveDate::from_ymd(2022, 1, 31));
//!
//! // Intervals are also iterable because they always have a duration!
//! // they are inclusive so they return the current time span first
//!
//! let next = interval.next().unwrap();
//!
//! assert_eq!(next.start(), NaiveDate::from_ymd(2022, 1, 1));
//! assert_eq!(next.end(), NaiveDate::from_ymd(2022, 1, 31));
//!
//! let next = interval.next().unwrap();
//!
//! assert_eq!(next.start(), NaiveDate::from_ymd(2022, 2, 1));
//! assert_eq!(next.end(), NaiveDate::from_ymd(2022, 2, 28));
//!
//! ```
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
//! let mut interval = Interval::from_start(start, duration);
//! ```

pub mod duration;
pub mod event;
pub mod grouping;
pub mod interval;
pub mod recurrence;
pub mod util;

pub use crate::duration::RelativeDuration;
pub use crate::interval::{Interval, IntervalLike, UnboundedEndInterval, UnboundedStartInterval};
pub use crate::recurrence::Recurrence;
pub use crate::recurrence::Rule;
pub use crate::util::*;
