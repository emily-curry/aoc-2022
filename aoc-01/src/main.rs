mod elf_inventory;

use crate::elf_inventory::ElfInventory;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let mut elves: Vec<ElfInventory> = input
        .as_string()
        .split("\n\n")
        .map(ElfInventory::from)
        .collect();
    elves.sort();
    println!("Elf with most calories: {}", elves.last().unwrap());

    let sum = elves
        .iter()
        .rev()
        .take(3)
        .fold(0, |acc, val| acc + val.sum_calories());
    println!("Sum of calories on 3 elves with most food: {}", sum);
}
