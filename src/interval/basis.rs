use super::{base::BaseInterval, naive::NaiveInterval};

pub trait Basis: IntoIterator<Item = Self::Basis> {
    type Basis;

    fn calculate<I>(basis: Self::Basis, interval: I) -> NaiveInterval
    where
        I: BaseInterval;
}
