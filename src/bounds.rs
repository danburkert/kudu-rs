use std::ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

/// Like `std::ops::RangeBounds`, but permits access to the owned values.
pub trait IntoBounds<T> {
    fn into_bounds(self) -> (Bound<T>, Bound<T>);
}

impl<T> IntoBounds<T> for (Bound<T>, Bound<T>) {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        self
    }
}

impl<T> IntoBounds<T> for RangeFull {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        (Bound::Unbounded, Bound::Unbounded)
    }
}

impl<T> IntoBounds<T> for RangeFrom<T> {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        (Bound::Included(self.start), Bound::Unbounded)
    }
}

impl<T> IntoBounds<T> for RangeTo<T> {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        (Bound::Unbounded, Bound::Excluded(self.end))
    }
}

impl<T> IntoBounds<T> for Range<T> {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        (Bound::Included(self.start), Bound::Excluded(self.end))
    }
}

impl<T> IntoBounds<T> for RangeInclusive<T> {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        let (start, end) = self.into_inner();
        (Bound::Included(start), Bound::Included(end))
    }
}

impl<T> IntoBounds<T> for RangeToInclusive<T> {
    fn into_bounds(self) -> (Bound<T>, Bound<T>) {
        (Bound::Unbounded, Bound::Included(self.end))
    }
}
