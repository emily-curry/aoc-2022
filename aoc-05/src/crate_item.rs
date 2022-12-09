use std::fmt::{Display, Formatter, Write};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CrateItem {
    contents: char,
}

impl From<char> for CrateItem {
    fn from(input: char) -> Self {
        CrateItem { contents: input }
    }
}

impl Display for CrateItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.contents)?;
        Ok(())
    }
}
