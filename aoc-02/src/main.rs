extern crate core;

mod game_choice;
mod game_decision;
mod game_outcome;
mod game_round;
mod game_tournament;

use crate::game_tournament::GameTournament;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let game = GameTournament::from(input.to_lines());
    println!("Scoring as choices: {}", game.sum_scores_as_choice());
    println!("Scoring as outcomes: {}", game.sum_scores_as_outcome());
}
