mod distress_map;
mod map_point;
mod map_sensor;

use crate::distress_map::DistressMap;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let map = DistressMap::from(input.to_lines());

    println!(
        "Possible locations in row 2000000: {}",
        map.count_excluded_in_row(2000000)
    );

    let distress_point = map.find_distress_signal();
    println!(
        "Distress signal frequency: {}",
        distress_point.x * 4000000 + distress_point.y
    )
}
