pub struct MonkeyItem {
    pub value: usize,
}

impl From<usize> for MonkeyItem {
    fn from(value: usize) -> Self {
        MonkeyItem { value }
    }
}
