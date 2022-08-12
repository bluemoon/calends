pub mod base;
pub mod bound;

#[allow(clippy::module_inception)]
pub mod interval;

pub use base::IntervalLike;
pub use interval::Interval;
