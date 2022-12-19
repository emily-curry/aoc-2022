use crate::jet_direction::JetDirection;
use crate::rock_group::RockGroup;
use crate::rock_kind::RockKind;
use crate::rock_point::RockPoint;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter, Write};

#[derive(Debug)]
pub struct RockState {
    max_y: usize,
    tower: HashSet<RockPoint>,
    shapes: VecDeque<RockKind>,
    jets: VecDeque<JetDirection>,
}

impl RockState {
    pub fn drop_rock(&mut self) {
        let next = self.shapes.pop_front().unwrap();
        let mut rock = next.to_rock(self.max_y);
        self.shapes.push_back(next);

        // find looping pattern? maybe every time jets cycles?
        while self.step_rock(&mut rock) {}
        let max_y = *rock.points.iter().map(|(_, y)| y).max().unwrap();
        if max_y > self.max_y {
            self.max_y = max_y;
        }
        self.tower.extend(rock.points.iter());
    }

    pub fn get_height(&self) -> usize {
        self.max_y
    }

    pub fn prune(&mut self) {
        let min_y = *self.tower.iter().map(|(_, y)| y).min().unwrap();
        let limit = (min_y..=self.max_y)
            .filter(|y| (0usize..7).all(|x| self.tower.contains(&(x, *y))))
            .max();
        if let Some(l) = limit {
            self.tower.retain(|p| p.1 >= l);
        }
    }

    fn step_rock(&mut self, rock: &mut RockGroup) -> bool {
        let next_dir = self.jets.pop_front().unwrap();
        if self.can_move_h(&rock, &next_dir) {
            for mut p in rock.points.iter_mut() {
                match next_dir {
                    JetDirection::Left => p.0 -= 1,
                    JetDirection::Right => p.0 += 1,
                }
            }
        }
        self.jets.push_back(next_dir);

        let can_fall = self.can_move_v(rock);
        if can_fall {
            for mut p in rock.points.iter_mut() {
                p.1 -= 1
            }
        }

        can_fall
    }

    fn can_move_h(&self, rock: &RockGroup, dir: &JetDirection) -> bool {
        rock.points
            .iter()
            .map(|p| match dir {
                JetDirection::Left => p.0.checked_sub(1).map(|x| (x, p.1)),
                JetDirection::Right => {
                    let x = p.0 + 1;
                    match x >= 7 {
                        true => None,
                        false => Some((x, p.1)),
                    }
                }
            })
            .all(|p| p.map(|pz| !self.tower.contains(&pz)).unwrap_or(false))
    }

    fn can_move_v(&self, rock: &RockGroup) -> bool {
        rock.points
            .iter()
            .map(|p| {
                let y = p.1 - 1;
                match y == 0 {
                    true => None,
                    false => Some((p.0, y)),
                }
            })
            .all(|p| p.map(|pz| !self.tower.contains(&pz)).unwrap_or(false))
    }
}

impl From<&str> for RockState {
    fn from(input: &str) -> Self {
        let jets = input.chars().map(JetDirection::from).collect();
        let shapes = VecDeque::from([
            RockKind::Dash,
            RockKind::Cross,
            RockKind::Bracket,
            RockKind::Pole,
            RockKind::Square,
        ]);

        RockState {
            max_y: 0,
            tower: HashSet::new(),
            jets,
            shapes,
        }
    }
}

impl Display for RockState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let min_y = *self.tower.iter().map(|(_, y)| y).min().unwrap().max(&1);
        for y in (min_y..(self.max_y + 1)).rev() {
            f.write_char('|')?;
            for x in 0..7usize {
                let c = match self.tower.contains(&(x, y)) {
                    true => '#',
                    false => '.',
                };
                f.write_char(c)?;
            }
            f.write_str("|\n")?;
        }
        f.write_str("+-------+")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::rock_state::RockState;

    #[test]
    fn example_input() {
        let mut state = RockState::from(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        state.drop_rock();
        println!("{}", state);
        assert_eq!(state.get_height(), 1);
        state.drop_rock();
        println!("{}", state);
        assert_eq!(state.get_height(), 4);
        state.drop_rock();
        println!("{}", state);
        assert_eq!(state.get_height(), 6);
        state.drop_rock();
        println!("{}", state);
        assert_eq!(state.get_height(), 7);
        state.drop_rock();
        println!("{}", state);
        assert_eq!(state.get_height(), 9);
        state.drop_rock();
        println!("{}", state);
        assert_eq!(state.get_height(), 10);
        state.drop_rock();
        println!("{}", state);
        assert_eq!(state.get_height(), 13);
        for _ in 7..2022 {
            state.drop_rock();
        }
        assert_eq!(state.get_height(), 3068);
    }

    #[test]
    fn example_two() {
        let mut state = RockState::from(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        for _ in 0..1000000000000usize {
            state.drop_rock();
        }
        assert_eq!(state.get_height(), 1514285714288usize);
    }
}
