mod system_command;
mod system_directory;
mod system_node;
mod system_state;

use crate::system_command::{ListNode, SystemCommand};
use crate::system_state::SystemState;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();

    let mut commands: Vec<SystemCommand> = Vec::new();
    for line in input.to_lines() {
        if line.starts_with('$') {
            commands.push(SystemCommand::from(line));
        } else {
            let last_command = commands.last_mut().unwrap();
            match last_command {
                SystemCommand::List(result) => result.push(ListNode::from(line)),
                _ => panic!("Command has no output!"),
            };
        }
    }

    let mut state = SystemState::default();
    for command in commands {
        state.run(command);
    }

    let mut all_directories = state.root.flat_directories();
    all_directories.sort_by(|a, b| b.size().cmp(&a.size()));

    let small_directories: usize = all_directories
        .iter()
        .map(|dir| dir.size())
        .filter(|size| size <= &100_000usize)
        .sum();
    println!("Sum of all small directories: {}", small_directories);

    let current_space = 70000000 - all_directories.first().unwrap().size();
    let space_to_free = 30000000 - current_space;
    let first_largest = all_directories
        .iter()
        .rev()
        .find(|dir| dir.size() >= space_to_free)
        .unwrap();
    println!(
        "Smallest directory that can fix error: {}",
        first_largest.size()
    );
}
