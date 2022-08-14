pub mod bound;
#[allow(clippy::module_inception)]
pub mod closed;
pub mod iter;
pub mod like;
pub mod open;
pub mod serde;

pub use closed::Interval;
pub use like::IntervalLike;
pub use open::OpenInterval;
