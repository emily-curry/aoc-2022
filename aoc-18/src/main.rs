mod lava_point;
mod lava_scanner;

use crate::lava_scanner::LavaScanner;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let scanner = LavaScanner::from(input.to_lines());

    println!("Surface area of droplet: {}", scanner.get_surface_area());
    println!(
        "External surface area of droplet: {}",
        scanner.get_external_surface_area()
    );
}
