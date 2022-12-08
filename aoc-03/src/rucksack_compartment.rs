use crate::rucksack_item::RucksackItem;

pub struct RucksackCompartment {
    contents: Vec<RucksackItem>,
}

impl RucksackCompartment {
    pub fn iter(&self) -> impl Iterator<Item = &RucksackItem> {
        self.contents.iter()
    }

    pub fn contains(&self, item: &RucksackItem) -> bool {
        self.contents.contains(item)
    }
}

impl<T> From<T> for RucksackCompartment
where
    T: Iterator<Item = char>,
{
    fn from(input: T) -> Self {
        RucksackCompartment {
            contents: input.map(RucksackItem::from).collect(),
        }
    }
}
