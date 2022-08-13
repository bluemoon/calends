pub mod bound;
#[allow(clippy::module_inception)]
pub mod closed;
pub mod interval_like;
pub mod iter;
pub mod open;

pub use closed::Interval;
pub use interval_like::IntervalLike;
