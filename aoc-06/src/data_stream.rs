use std::collections::HashSet;

pub struct DataStream {
    value: Vec<char>,
}

impl DataStream {
    pub fn find_marker(&self, size: usize) -> usize {
        self.value
            .windows(size)
            .enumerate()
            .find(|(_, win)| {
                let set: HashSet<&char> = HashSet::from_iter(win.iter());
                set.len() == size
            })
            .map(|(r, _)| r + size)
            .unwrap()
    }
}

impl From<&str> for DataStream {
    fn from(input: &str) -> Self {
        DataStream {
            value: input.chars().collect(),
        }
    }
}
