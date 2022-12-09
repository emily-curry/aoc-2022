use aoc_core::includes::Includes;
use std::ops::RangeInclusive;

pub struct WorkerAssignment {
    range: RangeInclusive<u32>,
}

impl From<&str> for WorkerAssignment {
    fn from(input: &str) -> Self {
        let mut split = input.split('-');
        let start = split.next().unwrap().parse().unwrap();
        let end = split.next().unwrap().parse().unwrap();
        WorkerAssignment { range: start..=end }
    }
}

impl Includes<WorkerAssignment> for WorkerAssignment {
    fn includes(&self, other: &WorkerAssignment) -> bool {
        self.range.includes(&other.range)
    }
}
