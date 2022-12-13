mod height_map;

use crate::height_map::HeightMap;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let map = HeightMap::from(input.to_lines());
    let last_step = map.find_shortest_path(false);
    println!(
        "Fewest number of steps to reach highest point from start position: {}",
        last_step.step
    );

    let last_step_fuzzy = map.find_shortest_path(true);
    println!(
        "Fewest number of steps to reach highest point from any low position: {}",
        last_step_fuzzy.step
    );
}
