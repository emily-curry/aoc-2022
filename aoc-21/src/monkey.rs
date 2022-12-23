use std::fmt::{Display, Formatter, Write};

pub const ROOT_MONKEY: Monkey = Monkey {
    id: ['r', 'o', 'o', 't'],
};

pub const BALD_MONKEY: Monkey = Monkey {
    id: ['h', 'u', 'm', 'n'],
};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Monkey {
    id: [char; 4],
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let id = input.chars().collect::<Vec<char>>().try_into().unwrap();
        Monkey { id }
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0usize..4 {
            f.write_char(self.id[i])?;
        }
        Ok(())
    }
}
