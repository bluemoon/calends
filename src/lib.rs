//! # calends
//!
//! calends is a library for durations, intervals and other calendar related operations. It is
//! designed to work with chrono.
//!
//! # Durations of time
//!
//! A duration is a unit of time that has some ability to be applied to a date to produce another
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
//! Recurrence allows you to specify a ruleset for how events (dates) repeat in time.
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

pub mod duration;
pub mod event;
pub mod interval;
pub mod recurrence;
pub mod util;

pub use crate::duration::RelativeDuration;
pub use crate::recurrence::Recurrence;
pub use crate::recurrence::Rule;
pub use crate::util::*;
