use std::fmt::{Debug, Formatter, Write};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct ValveLocation {
    loc: usize,
}

impl From<&str> for ValveLocation {
    fn from(input: &str) -> Self {
        let mut chars = input.chars();
        let l = chars.next().unwrap() as u8 as usize;
        let r = chars.next().unwrap() as u8 as usize;
        let mut loc = 0usize;
        loc |= l;
        loc <<= 8u8;
        loc |= r;
        ValveLocation { loc }
    }
}

impl Debug for ValveLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let r = (self.loc) as u8 as char;
        let l = (self.loc >> 8u8) as u8 as char;
        f.write_char(l)?;
        f.write_char(r)?;
        Ok(())
    }
}
