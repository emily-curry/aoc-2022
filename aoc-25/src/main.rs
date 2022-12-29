mod snafu_number;

use crate::snafu_number::SnafuNumber;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let numbers: Vec<SnafuNumber> = input.to_lines().map(SnafuNumber::from).collect();

    let sum: SnafuNumber = numbers.iter().sum();
    println!("Sum of all fuel requirements: {}", sum);
}
