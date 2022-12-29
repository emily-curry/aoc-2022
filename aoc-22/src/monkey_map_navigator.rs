use crate::monkey_map::MonkeyMap;
use crate::monkey_map_instruction::{MonkeyMapInstruction, MonkeyMapTurn};
use crate::monkey_map_state::MonkeyMapState;

pub struct MonkeyMapNavigator {
    map: MonkeyMap,
    instructions: Vec<MonkeyMapInstruction>,
}

impl MonkeyMapNavigator {
    pub fn follow_instructions(&self, as_cube: bool) -> MonkeyMapState {
        let initial_position = self.map.find_initial_point();
        let mut state = MonkeyMapState::new(initial_position);

        for inst in self.instructions.iter() {
            match inst {
                MonkeyMapInstruction::Advance(steps) => {
                    let (next_position, next_facing) = self.map.find_next_point(
                        state.get_position(),
                        state.get_facing(),
                        steps,
                        as_cube,
                    );
                    state.set_position(next_position);
                    state.set_facing(next_facing);
                }
                MonkeyMapInstruction::Turn(dir) => {
                    state.turn(dir);
                }
            }
        }

        state
    }
}

impl From<String> for MonkeyMapNavigator {
    fn from(input: String) -> Self {
        let lines_count = input.lines().count();
        let map = MonkeyMap::from(input.lines().take(lines_count - 2));
        let mut inst = input.lines().skip(lines_count - 1).next().unwrap().chars();
        let mut instructions: Vec<MonkeyMapInstruction> = Vec::new();
        let mut steps: Option<String> = None;
        while let Some(c) = inst.next() {
            if c.is_digit(10) {
                if let Some(s) = steps {
                    steps = Some(s + c.to_string().as_str());
                } else {
                    steps = Some(c.to_string());
                }
            } else {
                if let Some(s) = steps {
                    instructions.push(MonkeyMapInstruction::Advance(s.parse().unwrap()));
                    steps = None;
                }
                instructions.push(MonkeyMapInstruction::Turn(MonkeyMapTurn::from(c)));
            }
        }
        if let Some(s) = steps {
            instructions.push(MonkeyMapInstruction::Advance(s.parse().unwrap()));
        }

        MonkeyMapNavigator { map, instructions }
    }
}

#[cfg(test)]
mod tests {
    use crate::monkey_map_navigator::MonkeyMapNavigator;

    fn get_input() -> MonkeyMapNavigator {
        let input = r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"
            .to_string();
        input.into()
    }

    #[test]
    fn example1() {
        let input = get_input();
        let last_position = input.follow_instructions(false);
        assert_eq!(last_position.get_password(), 6032);
    }

    #[test]
    fn example2() {
        let input = get_input();
        let last_position = input.follow_instructions(true);
        assert_eq!(last_position.get_password(), 5031);
    }
}
