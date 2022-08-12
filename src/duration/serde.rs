use serde::{ser::SerializeStruct, Serialize, Serializer};

use crate::RelativeDuration;

/// Serialize a `RelativeDuration` as a human readable struct
impl Serialize for RelativeDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RelativeDuration", 3)?;
        state.serialize_field("months", &self.num_months())?;
        state.serialize_field("weeks", &self.num_weeks())?;
        state.serialize_field("days", &self.num_days())?;
        state.end()
    }
}

/// Used to serialize/deserialize from nanosecond-precision timestamps
///
/// # Example:
///
/// ```rust
/// # use calends::duration::RelativeDuration;
/// # use serde_derive::{Deserialize, Serialize};
/// use calends::duration::serde::rd_iso8601::serialize;
///
/// #[derive(Serialize)]
/// struct S {
///     #[serde(serialize_with = "serialize")]
///     duration: RelativeDuration
/// }
/// ```
pub mod rd_iso8601 {
    use serde::ser;

    use crate::RelativeDuration;

    /// Serialize a relative duration into an iso8601 duration
    ///
    /// Intended for use with `serde`s `serialize_with` attribute.
    ///
    /// # Example:
    /// ```rust
    /// # use calends::duration::RelativeDuration;
    /// # use serde_derive::{Deserialize, Serialize};
    /// use calends::duration::serde::rd_iso8601::serialize;
    ///
    /// #[derive(Serialize)]
    /// struct S {
    ///     #[serde(serialize_with = "serialize")]
    ///     duration: RelativeDuration
    /// }
    ///
    /// let s = S {
    ///     duration: RelativeDuration::months(3).with_days(-3)
    /// };
    /// let as_string = serde_json::to_string(&s)?;
    /// assert_eq!(as_string, r#"{"duration":"P3M-3D"}"#);
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    pub fn serialize<S>(rd: &RelativeDuration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&rd.iso8601())
    }
}
