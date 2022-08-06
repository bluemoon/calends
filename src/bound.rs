use std::{cmp::Ordering, ops::Bound};

pub fn cmp_bound<Q>(e1: &Bound<Q>, e2: &Bound<Q>) -> Ordering
where
  Q: Ord,
{
  // Based on the encoding idea used in `cmp`.
  // Note that we have inversed the 2nd value in the tuple,
  // as the Included/Excluded rules are flipped for the upper bound.
  let e1 = match e1 {
    Bound::Included(x) => Some((x, 2)),
    Bound::Excluded(x) => Some((x, 1)),
    Bound::Unbounded => None,
  };
  let e2 = match e2 {
    Bound::Included(x) => Some((x, 2)),
    Bound::Excluded(x) => Some((x, 1)),
    Bound::Unbounded => None,
  };

  match (e1, e2) {
    (None, None) => Ordering::Equal,
    (None, Some(_)) => Ordering::Greater,
    (Some(_), None) => Ordering::Less,
    (Some(r1), Some(ref r2)) => r1.cmp(r2),
  }
}

pub fn cmp_range<Q>(e1: (&Bound<Q>, &Bound<Q>), e2: (&Bound<Q>, &Bound<Q>)) -> Ordering
where
  Q: Ord,
{
  match cmp_bound(e1.0, e2.0) {
    Ordering::Less => Ordering::Less,
    Ordering::Equal => cmp_bound(e1.1, e2.1),
    Ordering::Greater => Ordering::Greater,
  }
}

pub fn within<Q>(item: Q, start: &Bound<Q>, end: &Bound<Q>) -> bool
where
  Q: Ord,
{
  let item_bound = Bound::Included(item);
  match cmp_bound(&item_bound, start) {
    Ordering::Less => false,
    Ordering::Equal => true,
    Ordering::Greater => match cmp_bound(&item_bound, end) {
      Ordering::Less => true,
      Ordering::Equal => true,
      Ordering::Greater => false,
    },
  }
}

pub fn to_str<Q>(b: Bound<Q>) -> String {
  match b {
    Bound::Included(_) => "inclusive".to_string(),
    Bound::Excluded(_) => "exclusive".to_string(),
    Bound::Unbounded => "unbounded".to_string(),
  }
}

pub fn to_opt<Q>(b: Bound<Q>) -> Option<Q> {
  match b {
    Bound::Included(q) => Some(q),
    Bound::Excluded(q) => Some(q),
    Bound::Unbounded => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_within() {
    assert!(within(3, &Bound::Included(1), &Bound::Unbounded))
  }

  #[test]
  fn test_cmp_range() {
    assert_eq!(
      cmp_range(
        (&Bound::Included(1), &Bound::Unbounded),
        (&Bound::Included(0), &Bound::Unbounded)
      ),
      Ordering::Greater
    )
  }
}
