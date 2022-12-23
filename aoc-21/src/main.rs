extern crate core;

mod monkey;
mod monkey_expression;
mod monkey_expression_tree;
mod monkey_operator;
mod monkey_riddle;

use crate::monkey_riddle::MonkeyRiddle;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let riddle = MonkeyRiddle::from(input.to_lines());
    println!("Root monkey will yell: {}", riddle.solve_for_root());
    println!("Bald monkey will yell: {}", riddle.solve_for_human());
}
