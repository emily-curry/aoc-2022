use std::fmt::{Display, Formatter, Write};

pub enum MonkeyMapTile {
    Void,
    Wall,
    Open,
}

impl From<char> for MonkeyMapTile {
    fn from(input: char) -> Self {
        match input {
            ' ' => MonkeyMapTile::Void,
            '#' => MonkeyMapTile::Wall,
            '.' => MonkeyMapTile::Open,
            _ => panic!("No such tile"),
        }
    }
}

impl Display for MonkeyMapTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MonkeyMapTile::Void => f.write_char(' '),
            MonkeyMapTile::Wall => f.write_char('#'),
            MonkeyMapTile::Open => f.write_char('.'),
        }
    }
}
