use core::fmt;

/// Month, day of month: `(month << 9) | (day << 4)`
#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct MonthDay(pub u32);

impl MonthDay {
  #[inline]
  fn clamp_month(month: u32) -> u32 {
    if month > 12 {
      0
    } else {
      month
    }
  }

  #[inline]
  fn clamp_day(day: u32) -> u32 {
    if day > 31 {
      0
    } else {
      day
    }
  }

  #[inline]
  pub fn new(month: u32, day: u32) -> MonthDay {
    let month = MonthDay::clamp_month(month);
    let day = MonthDay::clamp_day(day);
    MonthDay((month << 9) | (day << 4))
  }

  #[inline]
  pub fn month(&self) -> u32 {
    let MonthDay(mdf) = *self;
    mdf >> 9
  }

  #[inline]
  pub fn with_month(&self, month: u32) -> MonthDay {
    let month = MonthDay::clamp_month(month);
    let MonthDay(mdf) = *self;
    MonthDay((mdf & 0b1_1111_1111) | (month << 9))
  }

  #[inline]
  pub fn day(&self) -> u32 {
    let MonthDay(mdf) = *self;
    (mdf >> 4) & 0b1_1111
  }

  #[inline]
  pub fn with_day(&self, day: u32) -> MonthDay {
    let day = MonthDay::clamp_day(day);
    let MonthDay(mdf) = *self;
    MonthDay((mdf & !0b1_1111_0000) | (day << 4))
  }
}

impl fmt::Debug for MonthDay {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let MonthDay(mdf) = *self;
    write!(
      f,
      "Mdf(({} << 9) | ({} << 4))",
      mdf >> 9,
      (mdf >> 4) & 0b1_1111,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_month_day_creation() {
    let md = MonthDay::new(12, 31);
    assert_eq!(md.month(), 12);
    assert_eq!(md.day(), 31);
  }
}
