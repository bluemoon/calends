use chrono::NaiveDate;

use crate::addition;

/// The basis upon which we repeat
#[derive(
  Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Basis {
  Once,
  /// Given an a daily interval, this event happens the day of the interval and is always the same.
  ///
  /// - Offset: None
  /// - Begin: Same
  /// - End: Same
  Day,
  /// The next event happens on a specific day of the week
  ///
  /// - Offset: Day of week
  /// - Begin: Day of week
  /// - End: Day of week
  Week,
  ///
  Month,
  Quarter,
  Year,
}

/// An event happens on a date and can be recurring but depends on the basis
#[derive(Debug, Clone)]
pub struct Event {
  date: NaiveDate,
  basis: Basis,
}

impl Event {
  pub fn new(date: NaiveDate, basis: Basis) -> Self {
    Self { date, basis }
  }

  pub fn until(&self, date: NaiveDate) -> EventUntil {
    EventUntil::new(date, self.clone().into_iter())
  }
}

impl IntoIterator for Event {
  type Item = NaiveDate;

  type IntoIter = EventIntoIter;

  fn into_iter(self) -> Self::IntoIter {
    EventIntoIter::new(self)
  }
}

#[derive(Debug, Clone)]
pub struct EventIntoIter {
  event: Option<Event>,
}

impl EventIntoIter {
  pub fn new(event: Event) -> Self {
    Self { event: Some(event) }
  }
}

impl Iterator for EventIntoIter {
  type Item = NaiveDate;

  fn next(&mut self) -> Option<Self::Item> {
    match self.event.as_ref()?.basis {
      Basis::Once => self.event.take().map(|e| e.date),
      Basis::Day => {
        self.event = self
          .event
          .as_ref()
          .map(|event| Event::new(addition::add_day(event.date), event.basis));

        Some(self.event.as_ref()?.date)
      }
      Basis::Week => {
        self.event = self
          .event
          .as_ref()
          .map(|event| Event::new(addition::add_week_duration(event.date), event.basis));

        Some(self.event.as_ref()?.date)
      }
      Basis::Month => {
        self.event = self
          .event
          .as_ref()
          .map(|event| Event::new(addition::add_month_duration(event.date), event.basis));

        Some(self.event.as_ref()?.date)
      }
      Basis::Quarter => {
        self.event = self
          .event
          .as_ref()
          .map(|event| Event::new(addition::add_quarter_duration(event.date), event.basis));

        Some(self.event.as_ref()?.date)
      }
      Basis::Year => {
        self.event = self
          .event
          .as_ref()
          .map(|event| Event::new(addition::add_year_duration(event.date), event.basis));

        Some(self.event.as_ref()?.date)
      }
    }
  }
}

#[derive(Debug, Clone)]
pub struct EventUntil {
  pub until: NaiveDate,
  pub event_iter: EventIntoIter,
}

impl EventUntil {
  pub fn new(until: NaiveDate, event_iter: EventIntoIter) -> Self {
    Self { until, event_iter }
  }
}

impl Iterator for EventUntil {
  type Item = NaiveDate;

  fn next(&mut self) -> Option<Self::Item> {
    let event = self.event_iter.next()?;
    if event <= self.until {
      Some(event)
    } else {
      None
    }
  }
}
