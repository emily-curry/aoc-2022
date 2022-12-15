use crate::map_container::MapContainer;
use aoc_core::puzzle_input::PuzzleInput;

mod map_container;
mod map_content;
mod map_line;
mod map_point;

fn main() {
    let input = PuzzleInput::default();
    let mut map = MapContainer::from(input.to_lines());
    map.fill();
    println!(
        "All sand units at rest assuming infinite void: {}",
        map.count_sand()
    );

    map.set_infinite(false);
    map.fill();
    println!(
        "All sand units at rest after blocking source: {}",
        map.count_sand()
    );
}
