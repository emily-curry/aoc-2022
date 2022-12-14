extern crate core;

use crate::list_packet_pair::ListPacketList;
use aoc_core::puzzle_input::PuzzleInput;

mod list_packet;
mod list_packet_pair;

fn main() {
    let input = PuzzleInput::default();
    let mut packets = ListPacketList::from(input.as_string().as_str());

    let ordered_sum: usize = packets.ordered_indices().sum();
    println!("Sum of indices of correctly ordered pairs: {}", ordered_sum);

    packets.insert_divider_packets();
    let point = packets.find_correct_order();
    println!("Product of divider packet points: {}", point.0 * point.1)
}
