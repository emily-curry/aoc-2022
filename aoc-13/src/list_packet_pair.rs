use crate::list_packet::ListPacket;
use std::cmp::Ordering;

pub struct ListPacketPair<'a> {
    left: &'a ListPacket,
    right: &'a ListPacket,
}

impl<'a> ListPacketPair<'a> {
    pub fn is_ordered(&self) -> bool {
        matches!(self.left.cmp(&self.right), Ordering::Less)
    }

    pub fn new(left: &'a ListPacket, right: &'a ListPacket) -> Self {
        ListPacketPair { left, right }
    }
}

pub struct ListPacketList {
    packets: Vec<ListPacket>,
}

impl ListPacketList {
    pub fn ordered_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.packets
            .chunks(2)
            .map(|sl| ListPacketPair::new(&sl[0], &sl[1]))
            .enumerate()
            .filter(|(_, p)| p.is_ordered())
            .map(|(i, _)| i + 1)
    }

    pub fn insert_divider_packets(&mut self) {
        self.packets.push(ListPacket::from("[[2]]"));
        self.packets.push(ListPacket::from("[[6]]"));
    }

    pub fn find_correct_order(&mut self) -> (usize, usize) {
        self.packets.sort();
        if !self.is_ordered() {
            panic!("Not actually ordered like expected!");
        }
        let a = self
            .packets
            .iter()
            .enumerate()
            .find(|(_, p)| *p == &ListPacket::from("[[2]]"))
            .map(|(i, _)| i)
            .unwrap();
        let b = self
            .packets
            .iter()
            .enumerate()
            .find(|(_, p)| *p == &ListPacket::from("[[6]]"))
            .map(|(i, _)| i)
            .unwrap();
        (a + 1, b + 1)
    }

    fn is_ordered(&self) -> bool {
        self.packets
            .chunks(2)
            .map(|sl| ListPacketPair::new(&sl[0], &sl[1]))
            .all(|p| p.is_ordered())
    }
}

impl From<&str> for ListPacketList {
    fn from(input: &str) -> Self {
        let packets = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(ListPacket::from)
            .collect();

        ListPacketList { packets }
    }
}
