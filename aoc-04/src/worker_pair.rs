use crate::worker_assignment::WorkerAssignment;
use aoc_core::includes::Includes;

pub struct WorkerPair {
    a: WorkerAssignment,
    b: WorkerAssignment,
}

impl WorkerPair {
    pub fn does_include(&self) -> bool {
        self.a.includes(&self.b) || self.b.includes(&self.a)
    }
}

impl From<&str> for WorkerPair {
    fn from(input: &str) -> Self {
        let mut split = input.split(',');
        let a = WorkerAssignment::from(split.next().unwrap());
        let b = WorkerAssignment::from(split.next().unwrap());
        WorkerPair { a, b }
    }
}
