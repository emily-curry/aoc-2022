mod rope_grid;
mod rope_movement;

use crate::rope_grid::RopeGrid;
use crate::rope_movement::RopeMovement;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let movements: Vec<RopeMovement> = input.to_lines().map(RopeMovement::from).collect();
    let mut rope = RopeGrid::default();
    for movement in &movements {
        rope.move_head(movement);
    }
    let unique_tail_points = rope.tail_visited.len();
    println!(
        "Size 2 rope tail visited {} unique points",
        unique_tail_points
    );

    let mut rope = RopeGrid::new(10);
    for movement in &movements {
        rope.move_head(movement);
    }
    let unique_tail_points = rope.tail_visited.len();
    println!(
        "Size 10 rope tail visited {} unique points",
        unique_tail_points
    );
}
