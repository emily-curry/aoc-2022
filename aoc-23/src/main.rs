mod elf_map;

use crate::elf_map::ElfMap;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let mut map = ElfMap::from(input.to_lines());

    for _ in 0..10 {
        map.step();
    }
    println!("Empty tiles in covered area: {}", map.count_empty());

    while map.step() {}
    println!(
        "First round where no elves moved: {}",
        map.get_current_step()
    );
}
