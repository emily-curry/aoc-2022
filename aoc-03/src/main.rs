mod elf_group;
mod rucksack;
mod rucksack_compartment;
mod rucksack_item;

use crate::elf_group::ElfGroup;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let groups: Vec<ElfGroup> = input
        .to_lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(ElfGroup::from)
        .collect();
    let sum_misplaced = groups
        .iter()
        .map(|r| r.sum_misplaced_items())
        .fold(0u64, |acc, val| acc + val);
    println!("Sum of misplaced items priorities: {}", sum_misplaced);

    let sum_badges = groups
        .iter()
        .map(|x| x.find_badge().get_priority())
        .fold(0u64, |acc, val| acc + val as u64);
    println!("Sum of badge priorities: {}", sum_badges);
}
