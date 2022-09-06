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

/// Used to serialize/deserialize from ISO8601-2:2019 Durations
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
    use serde::{de, ser};

    use crate::{duration::parse::parse_relative_duration, RelativeDuration};

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

    /// Deserialize a `RelativeDuration` from an ISO8601-2 duration
    ///
    /// Intended for use with `serde`s `deserialize_with` attribute.
    ///
    /// # Example:
    ///
    /// ```rust
    /// # use calends::duration::RelativeDuration;
    /// # use serde_derive::{Deserialize, Serialize};
    /// use calends::duration::serde::rd_iso8601::deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct S {
    ///     #[serde(deserialize_with = "deserialize")]
    ///     rd: RelativeDuration
    /// }
    ///
    /// let my_s: S = serde_json::from_str(r#"{ "rd": "P3M-3D" }"#)?;
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    pub fn deserialize<'de, D>(d: D) -> Result<RelativeDuration, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_string(DurationVisitor)
    }

    pub struct DurationVisitor;

    impl<'de> de::Visitor<'de> for DurationVisitor {
        type Value = RelativeDuration;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a ISO8601-2:2019 duration")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            parse_relative_duration(v)
                .map(|(_, d)| d)
                .map_err(E::custom)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        #[derive(Debug, serde::Deserialize, serde::Serialize)]
        struct S {
            #[serde(
                deserialize_with = "rd_iso8601::deserialize",
                serialize_with = "rd_iso8601::serialize"
            )]
            rd: RelativeDuration,
        }

        let rd = RelativeDuration::default().with_days(1).with_months(23);
        let s = S { rd };
        let parsed: S = serde_json::from_str(&serde_json::to_string(&s).unwrap()).unwrap();
        assert_eq!(rd, parsed.rd)
    }
}
