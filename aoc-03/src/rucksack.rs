use crate::rucksack_compartment::RucksackCompartment;
use crate::rucksack_item::RucksackItem;

pub struct Rucksack {
    left: RucksackCompartment,
    right: RucksackCompartment,
}

impl Rucksack {
    pub fn find_misplaced_item(&self) -> RucksackItem {
        for item in self.left.iter() {
            if self.right.contains(item) {
                return *item;
            }
        }
        panic!("No item found in both compartments!")
    }

    pub fn contains(&self, item: &RucksackItem) -> bool {
        self.left.contains(item) || self.right.contains(item)
    }

    pub fn iter(&self) -> impl Iterator<Item = &RucksackItem> {
        self.left.iter().chain(self.right.iter())
    }
}

impl From<&str> for Rucksack {
    fn from(input: &str) -> Self {
        let midpoint = input.len() / 2; // Assumes always even
        let left = input[0..midpoint].chars().into();
        let right = input[midpoint..].chars().into();

        Rucksack { left, right }
    }
}
