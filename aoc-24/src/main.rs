use crate::blizzard_navigator::BlizzardNavigator;
use aoc_core::puzzle_input::PuzzleInput;

mod blizzard_map;
mod blizzard_navigator;
mod blizzard_navigator_state;

fn main() {
    let input = PuzzleInput::default();
    let nav = BlizzardNavigator::from(input.as_string().as_str());
    let shortest_path = nav.find_shortest_path();
    println!("Shortest path: {}", shortest_path.0);
    println!(
        "Shortest path, there and back again (and there again): {}",
        shortest_path.1
    );
}
