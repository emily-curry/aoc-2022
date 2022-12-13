mod monkey;
mod monkey_decision;
mod monkey_expression;
mod monkey_group;
mod monkey_item;

use crate::monkey_group::MonkeyGroup;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let mut monkey_group = MonkeyGroup::from(input.as_string().as_str());
    for _ in 0..20 {
        monkey_group.perform_round(true);
    }

    println!(
        "Product of 2 most active monkeys inspections after 20 rounds: {}",
        monkey_group.most_active_score()
    );

    let mut monkey_group_no_reduce = MonkeyGroup::from(input.as_string().as_str());
    for _ in 0..10000 {
        monkey_group_no_reduce.perform_round(false);
    }
    println!(
        "Product of 2 most active monkeys inspections after 10000 rounds: {}",
        monkey_group_no_reduce.most_active_score()
    );
}
