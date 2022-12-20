use crate::rock_state::RockState;
use aoc_core::puzzle_input::PuzzleInput;

mod jet_direction;
mod rock_group;
mod rock_kind;
mod rock_point;
mod rock_state;

const TARGET: usize = 1000000000000usize;

fn main() {
    let input = PuzzleInput::default();
    let mut state = RockState::from(input.as_string().as_str());
    for _ in 0..2022 {
        state.drop_rock();
    }

    println!("Height of tower after 2022 rocks: {}", state.max_y);

    let mut state = RockState::from(input.as_string().as_str());
    let mut i = 0usize;
    let (initial_loop_step, initial_loop_height) = drop_until_loop(&mut i, &mut state);
    let (next_loop_step, next_loop_height) = drop_until_loop(&mut i, &mut state);
    let d_step = next_loop_step - initial_loop_step;
    let d_height = next_loop_height - initial_loop_height;

    let hyperspeed_iterations = (TARGET - i) / d_step;
    i += hyperspeed_iterations * d_step;
    state.add_height(hyperspeed_iterations * d_height);

    while i < TARGET {
        state.drop_rock();
        i += 1;
    }
    println!("Height of tower after 1000000000000 rocks: {}", state.max_y);
}

fn drop_until_loop(i: &mut usize, state: &mut RockState) -> (usize, usize) {
    loop {
        let did_loop = state.drop_rock();
        *i += 1;
        if did_loop {
            break (*i, state.max_y);
        }
        if *i % 100 == 0 {
            state.prune();
        }
    }
}
