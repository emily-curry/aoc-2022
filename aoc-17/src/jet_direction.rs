#[derive(Debug, Copy, Clone)]
pub enum JetDirection {
    Left,
    Right,
}

impl From<char> for JetDirection {
    fn from(input: char) -> Self {
        match input {
            '>' => JetDirection::Right,
            '<' => JetDirection::Left,
            _ => panic!(),
        }
    }
}
