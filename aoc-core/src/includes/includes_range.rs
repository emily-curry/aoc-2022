use crate::includes::Includes;
use std::ops::{Range, RangeInclusive};

impl<T> Includes<Range<T>> for Range<T>
where
    T: PartialOrd,
{
    fn includes(&self, other: &Range<T>) -> bool {
        other.start >= self.start && other.end <= self.end
    }
}

impl<T> Includes<RangeInclusive<T>> for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn includes(&self, other: &RangeInclusive<T>) -> bool {
        other.start() >= self.start() && other.end() <= self.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::includes::Includes;

    #[test]
    fn includes_range() {
        let r1 = 10..20;
        let r2 = 9..40;
        let r3 = 15..30;
        assert_eq!(r2.includes(&r1), true);
        assert_eq!(r1.includes(&r2), false);
        assert_eq!(r2.includes(&r3), true);
        assert_eq!(r1.includes(&r3), false);
    }

    #[test]
    fn includes_range_inclusive() {
        let r1 = 50..=100;
        let r2 = 0..=100;
        let r3 = 15..=101;
        assert_eq!(r2.includes(&r1), true);
        assert_eq!(r1.includes(&r2), false);
        assert_eq!(r2.includes(&r3), false);
        assert_eq!(r1.includes(&r3), false);
        assert_eq!(r3.includes(&r1), true);
    }
}
