#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RucksackItem {
    value: char,
}

impl RucksackItem {
    pub fn get_priority(&self) -> u8 {
        let code = self.value as u8;
        if code >= 'a' as u8 && code <= 'z' as u8 {
            code - 96
        } else if code >= 'A' as u8 && code <= 'Z' as u8 {
            code - 38
        } else {
            panic!("Invalid char: {}", self.value)
        }
    }
}

impl From<char> for RucksackItem {
    fn from(input: char) -> Self {
        RucksackItem { value: input }
    }
}

#[cfg(test)]
mod tests {
    use crate::rucksack_item::RucksackItem;

    #[test]
    fn converts_to_priority() {
        let a_lower = RucksackItem::from('a');
        assert_eq!(a_lower.get_priority(), 1);

        let z_lower = RucksackItem::from('z');
        assert_eq!(z_lower.get_priority(), 26);

        let a_upper = RucksackItem::from('A');
        assert_eq!(a_upper.get_priority(), 27);

        let z_upper = RucksackItem::from('Z');
        assert_eq!(z_upper.get_priority(), 52);
    }
}
