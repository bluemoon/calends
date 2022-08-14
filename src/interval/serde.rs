use serde::{ser::SerializeStruct, Serialize, Serializer};

use crate::{Interval, IntervalLike};

use super::open::OpenInterval;

/// Serialize a `Interval` as a human readable struct
impl Serialize for Interval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Interval", 2)?;
        state.serialize_field("start", &self.start_date())?;
        state.serialize_field("end", &self.end_date())?;
        state.end()
    }
}

/// Used to serialize Interval into an iso8601 format
///
/// # Example:
///
/// ```rust
/// # use calends::Interval;
/// # use serde_derive::{Deserialize, Serialize};
/// # use chrono::NaiveDate;
/// use calends::interval::serde::int_iso8601::serialize;
///
/// #[derive(Serialize)]
/// struct S {
///     #[serde(serialize_with = "serialize")]
///     interval: Interval
/// }
/// ```
pub mod int_iso8601 {
    use serde::ser;

    use crate::Interval;

    /// Serialize a relative duration into an iso8601 duration
    ///
    /// Intended for use with `serde`s `serialize_with` attribute.
    ///
    /// # Example:
    /// ```rust
    /// # use calends::{Interval, RelativeDuration};
    /// # use serde_derive::{Deserialize, Serialize};
    /// # use chrono::NaiveDate;
    /// use calends::interval::serde::int_iso8601::serialize;
    ///
    /// #[derive(Serialize)]
    /// struct S {
    ///     #[serde(serialize_with = "serialize")]
    ///     interval: Interval
    /// }
    ///
    /// let s = S {
    ///     interval: Interval::from_start(
    ///         NaiveDate::from_ymd(2022, 1, 1),
    ///         RelativeDuration::months(3).with_days(-3)
    ///     ),
    /// };
    /// let as_string = serde_json::to_string(&s)?;
    /// assert_eq!(as_string, r#"{"interval":"2022-01-01/2022-03-29"}"#);
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    pub fn serialize<S>(int: &Interval, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&int.iso8601())
    }
}

/// Serialize a `OpenInterval` as a human readable struct
impl Serialize for OpenInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Interval", 2)?;
        state.serialize_field("start", &self.start_date())?;
        state.serialize_field("end", &self.end_date())?;
        state.end()
    }
}

/// Used to serialize Interval into an iso8601 format
///
/// # Example:
///
/// ```rust
/// # use calends::OpenInterval;
/// # use serde_derive::{Deserialize, Serialize};
/// # use chrono::NaiveDate;
/// use calends::interval::serde::oint_iso8601::serialize;
///
/// #[derive(Serialize)]
/// struct S {
///     #[serde(serialize_with = "serialize")]
///     interval:OpenInterval
/// }
/// ```
pub mod oint_iso8601 {
    use serde::ser;

    use crate::OpenInterval;

    /// Serialize a relative duration into an iso8601 duration
    ///
    /// Intended for use with `serde`s `serialize_with` attribute.
    ///
    /// # Example:
    /// ```rust
    /// # use calends::{OpenInterval, RelativeDuration};
    /// # use serde_derive::{Deserialize, Serialize};
    /// # use chrono::NaiveDate;
    /// use calends::interval::serde::oint_iso8601::serialize;
    ///
    /// #[derive(Serialize)]
    /// struct S {
    ///     #[serde(serialize_with = "serialize")]
    ///     interval: OpenInterval
    /// }
    ///
    /// let s = S {
    ///     interval: OpenInterval::Start(
    ///         NaiveDate::from_ymd(2022, 1, 1),
    ///     ),
    /// };
    /// let as_string = serde_json::to_string(&s)?;
    /// assert_eq!(as_string, r#"{"interval":"../2022-01-01"}"#);
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    pub fn serialize<S>(int: &OpenInterval, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&int.iso8601())
    }
}
