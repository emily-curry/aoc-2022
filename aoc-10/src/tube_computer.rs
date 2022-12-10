use crate::tube_instruction::TubeInstruction;
use std::fmt::{Display, Formatter, Write};

pub struct TubeComputer {
    pub acc: isize,
    pub cycle: usize,
    screen: [[bool; 40]; 6],
}

impl TubeComputer {
    pub fn run<'a, T, R, F>(&mut self, instructions: T, debug: F) -> Vec<R>
    where
        T: Iterator<Item = &'a TubeInstruction>,
        F: Fn(&TubeComputer) -> Option<R>,
    {
        let mut result: Vec<R> = Vec::new();
        for inst in instructions {
            for _ in 0..inst.cycles() {
                let sprite = (self.acc - 1)..=(self.acc + 1);
                let col = self.cycle % 40;
                if sprite.contains(&(col as isize)) {
                    let row = (self.cycle - col) / 40;
                    self.screen[row][col] = true;
                }

                self.cycle += 1;
                if let Some(i) = debug(self) {
                    result.push(i);
                }
            }
            match inst {
                TubeInstruction::NoOp => {}
                TubeInstruction::AddX(val) => self.acc += val,
            }
        }
        result
    }
}

impl Default for TubeComputer {
    fn default() -> Self {
        TubeComputer {
            acc: 1,
            cycle: 0,
            screen: [[false; 40]; 6],
        }
    }
}

impl Display for TubeComputer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.screen.iter() {
            for cell in row {
                let char = match cell {
                    true => '#',
                    false => '.',
                };
                f.write_char(char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
