mod valve;
mod valve_location;
mod valve_map;
mod valve_map_state;

use crate::valve_map::ValveMap;
use crate::valve_map_state::ValveMapStateContainer;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let map = ValveMap::from(input.to_lines());
    let state = ValveMapStateContainer::new(map);

    let max_pressure = state.find_max_pressure(false);
    println!("Max pressure possible to release: {}", max_pressure);

    let max_pressure_with_help = state.find_max_pressure(true);
    println!(
        "Max pressure possible to release with help: {}",
        max_pressure_with_help
    );
}
