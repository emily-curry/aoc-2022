mod decrypter;

use crate::decrypter::Decrypter;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let mut decrypter = Decrypter::from(input.to_lines());

    let coords = decrypter.get_coords(1);
    println!("{:?}", coords);
    let csum = coords.0 + coords.1 + coords.2;
    println!("Sum of GPS coords: {}", csum);

    decrypter.apply_key();
    let coords = decrypter.get_coords(10);
    let csum = coords.0 + coords.1 + coords.2;
    println!("Sum of GPS coords after applying key: {}", csum);
}
