mod worker_assignment;
mod worker_pair;

use crate::worker_pair::WorkerPair;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let pairs: Vec<WorkerPair> = input.to_lines().map(WorkerPair::from).collect();
    let full_overlaps = pairs.iter().filter(|x| x.does_include()).count();
    println!(
        "Pairs in which one assignment fully overlaps another: {}",
        full_overlaps
    );

    let partial_overlaps = pairs.iter().filter(|x| x.does_overlap()).count();
    println!(
        "Pairs in which one assignment partially overlaps another: {}",
        partial_overlaps
    );
}
