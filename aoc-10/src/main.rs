use crate::tube_computer::TubeComputer;
use crate::tube_instruction::TubeInstruction;
use aoc_core::puzzle_input::PuzzleInput;

mod tube_computer;
mod tube_instruction;

const CHECKPOINTS: [usize; 6] = [20, 60, 100, 140, 180, 220];

fn main() {
    let input = PuzzleInput::default();
    let instructions: Vec<TubeInstruction> = input.to_lines().map(TubeInstruction::from).collect();
    let mut cpu = TubeComputer::default();
    let debug_result: isize = cpu
        .run(instructions.iter(), |state| {
            match CHECKPOINTS.contains(&state.cycle) {
                true => Some(state.cycle as isize * state.acc),
                false => None,
            }
        })
        .iter()
        .sum();
    println!("Sum of checkpoint signal strengths: {}", debug_result);
    print!("{}", cpu);
}
