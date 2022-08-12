# calends

An opinionated calendar focused on calendars. This library is specifically focused on events that repeat and intervals of time such as months.

## Relative Durations

It is often useful to have durations of time that are months, weeks, days. Currently Chrono does not have support for month long
durations and we aim to fill that gap with this library.

```rust
use calends::RelativeDuration;
use chrono::NaiveDate;

// This will allow you to add one month and then go back one day from the added month
let rd = RelativeDuration::months(1).with_days(-1);

// It also compatible with NaiveDate
rd + NaiveDate::from_ymd(2022, 1, 1)
```

## Intervals

An interval of time is a start point plus a duration
