use crate::crate_crane::{CraneType, CrateCrane};
use aoc_core::puzzle_input::PuzzleInput;

mod crate_crane;
mod crate_instruction;
mod crate_item;
mod crate_stack;

fn main() {
    let input = PuzzleInput::default();
    let mut crane = CrateCrane::from(input.to_lines());
    println!("Initial crate arrangement:");
    println!("{}", crane);

    crane.process_instructions(CraneType::Single);
    println!("Final crate arrangement (single):");
    println!("{}", crane);
    println!("{}", crane.top_code());

    let mut crane = CrateCrane::from(input.to_lines());
    crane.process_instructions(CraneType::Stack);
    println!("Final crate arrangement (stack):");
    println!("{}", crane);
    println!("{}", crane.top_code());
}
