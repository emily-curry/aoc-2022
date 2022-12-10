mod data_stream;

use crate::data_stream::DataStream;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let stream = DataStream::from(input.as_string().as_str());
    println!("Start-of-packet marker: {}", stream.find_marker(4));
    println!("Start-of-packet marker: {}", stream.find_marker(14));
}
