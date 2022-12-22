mod robot_blueprint;
mod robot_factory;
mod robot_factory_command;
mod robot_factory_state;

use crate::robot_factory::RobotFactory;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let factories: Vec<RobotFactory> = input.to_lines().map(RobotFactory::from).collect();

    let quality_sum: usize = factories
        .iter()
        .map(|f| f.find_geodes_in_steps(24).geode * f.get_id())
        .sum();
    println!("Sum of all quality values: {}", quality_sum);

    let geode_product: usize = factories
        .iter()
        .filter(|f| f.get_id() <= 3)
        .map(|f| f.find_geodes_in_steps(32).geode)
        .product();
    println!(
        "Product of geodes produced by first 3 blueprints: {}",
        geode_product
    );
}
