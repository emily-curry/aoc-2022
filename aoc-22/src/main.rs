use crate::monkey_map_navigator::MonkeyMapNavigator;
use aoc_core::puzzle_input::PuzzleInput;

mod monkey_map;
mod monkey_map_instruction;
mod monkey_map_navigator;
mod monkey_map_state;
mod monkey_map_tile;

fn main() {
    let input = PuzzleInput::default();
    let nav = MonkeyMapNavigator::from(input.as_string().to_owned());

    let last_position = nav.follow_instructions(false);
    println!("Final password: {}", last_position.get_password());

    let last_position = nav.follow_instructions(true);
    println!("Final password as cube: {}", last_position.get_password());
}
