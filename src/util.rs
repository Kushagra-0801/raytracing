use std::ops::RangeInclusive;

pub trait RangeSurround<T> {
    fn surrounds(&self, val: &T) -> bool;
}

impl<T: PartialOrd> RangeSurround<T> for RangeInclusive<T> {
    fn surrounds(&self, val: &T) -> bool {
        self.start() < val && val < self.end()
    }
}
