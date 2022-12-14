use std::borrow::BorrowMut;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ListPacketItem {
    Value(usize),
    List(ListPacket),
}

impl PartialOrd for ListPacketItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for ListPacketItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ListPacketItem::Value(left), ListPacketItem::Value(right)) => left.cmp(right),
            (ListPacketItem::List(left), ListPacketItem::List(right)) => left.cmp(right),
            (ListPacketItem::List(left), ListPacketItem::Value(right)) => {
                left.cmp(&ListPacket::new(vec![ListPacketItem::Value(*right)]))
            }
            (ListPacketItem::Value(left), ListPacketItem::List(right)) => {
                ListPacket::new(vec![ListPacketItem::Value(*left)]).cmp(right)
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ListPacket {
    items: Vec<ListPacketItem>,
}

impl ListPacket {
    pub const fn new(items: Vec<ListPacketItem>) -> Self {
        ListPacket { items }
    }
}

impl PartialOrd for ListPacket {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for ListPacket {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        for i in 0usize.. {
            let left_item = if let Some(i) = self.items.get(i) {
                i
            } else {
                return Ordering::Less;
            };
            let right_item = if let Some(i) = other.items.get(i) {
                i
            } else {
                return Ordering::Greater;
            };
            let result = left_item.cmp(right_item);
            if !matches!(result, Ordering::Equal) {
                return result;
            }
        }
        panic!()
    }
}

impl From<&str> for ListPacket {
    fn from(input: &str) -> Self {
        let mut chars = input.chars().skip(1);
        ListPacket::from(&mut chars)
    }
}

impl<T> From<&mut T> for ListPacket
where
    T: Iterator<Item = char>,
{
    fn from(chars: &mut T) -> Self {
        let mut items: Vec<ListPacketItem> = Vec::new();
        let mut num_buffer = String::new();

        while let Some(char) = chars.next() {
            match char {
                ',' => {
                    if !num_buffer.is_empty() {
                        items.push(ListPacketItem::Value(num_buffer.parse().unwrap()));
                        num_buffer.clear();
                    }
                }
                '[' => items.push(ListPacketItem::List(ListPacket::from(chars.borrow_mut()))),
                ']' => {
                    if !num_buffer.is_empty() {
                        items.push(ListPacketItem::Value(num_buffer.parse().unwrap()));
                        num_buffer.clear();
                    }
                    break;
                }
                rest => num_buffer.push(rest),
            };
        }

        ListPacket { items }
    }
}

#[cfg(test)]
mod tests {
    use crate::list_packet::{ListPacket, ListPacketItem};
    use std::cmp::Ordering;

    #[test]
    fn from_str() {
        let packet = ListPacket::from("[[[7,8],5],[[9,[8,7,8],[],[2,4,10,10],[2,10,8,3,3]],[],[[6,1,10],[],3,6],[3]],[],[4],[3,0,1,10]]");
        let expected_first = ListPacket::new(vec![
            ListPacketItem::List(ListPacket::new(vec![
                ListPacketItem::Value(7),
                ListPacketItem::Value(8),
            ])),
            ListPacketItem::Value(5),
        ]);
        assert_eq!(packet.items[0], ListPacketItem::List(expected_first));

        let expected_last = ListPacket::new(vec![
            ListPacketItem::Value(3),
            ListPacketItem::Value(0),
            ListPacketItem::Value(1),
            ListPacketItem::Value(10),
        ]);
        assert_eq!(packet.items[4], ListPacketItem::List(expected_last));
    }

    #[test]
    fn packet_item_compare_value() {
        assert_eq!(
            ListPacketItem::Value(3).cmp(&ListPacketItem::Value(4)),
            Ordering::Less
        );

        assert_eq!(
            ListPacketItem::Value(10).cmp(&ListPacketItem::Value(3)),
            Ordering::Greater
        );

        assert_eq!(
            ListPacketItem::Value(5).cmp(&ListPacketItem::Value(5)),
            Ordering::Equal
        );
    }
}
