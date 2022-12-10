mod forest;
mod tree;

use crate::forest::Forest;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let forest = Forest::from(input.to_lines());

    let visible_trees = forest
        .iter_trees()
        .filter(|loc| forest.is_visible(loc))
        .count();
    println!("Trees visible from outside forest: {}", visible_trees);

    let highest_scenic_score = forest
        .iter_trees()
        .map(|loc| forest.get_scenic_score(&loc))
        .max()
        .unwrap();
    println!("Highest scenic score possible: {}", highest_scenic_score);
}
