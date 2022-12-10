#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Copy, Clone)]
pub struct Tree {
    height: u8,
}

impl From<char> for Tree {
    fn from(input: char) -> Self {
        let height = input.to_digit(10).unwrap() as u8;
        Tree { height }
    }
}
