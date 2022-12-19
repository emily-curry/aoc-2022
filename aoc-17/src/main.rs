use crate::rock_state::RockState;
use aoc_core::puzzle_input::PuzzleInput;

mod jet_direction;
mod rock_group;
mod rock_kind;
mod rock_point;
mod rock_state;

fn main() {
    let input = PuzzleInput::default();
    let mut state = RockState::from(input.as_string().as_str());
    for _ in 0..2022 {
        state.drop_rock();
    }

    println!("Height of tower after 2022 rocks: {}", state.get_height());
    let interval = 1000000000000usize / 10000;
    for i in 2022..1000000000000usize {
        state.drop_rock();
        if i % 100 == 0 {
            state.prune();
        }
        if i % interval == 0 {
            let amt = i as f64 / 1000000000000f64;
            println!("{}%", amt * 100f64);
        }
    }
    println!(
        "Height of tower after 1000000000000 rocks: {}",
        state.get_height()
    );
}
