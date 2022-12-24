use crate::robot_factory_command::RobotFactoryCommand;
use crate::robot_factory_state::RobotFactoryState;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RobotBlueprint {
    id: usize,
    /// Cost to build an ore robot, in terms of ore.
    ore_cost: usize,
    /// Cost to build a clay robot, in terms of ore.
    clay_cost: usize,
    /// Cost to build an obsidian robot, in terms of (ore, clay).
    obsidian_cost: (usize, usize),
    /// Cost to build a geode robot, in terms of (ore, obsidian).
    geode_cost: (usize, usize),
}

impl RobotBlueprint {
    pub fn find_next_states(
        &self,
        state: &RobotFactoryState,
        time_limit: usize,
    ) -> Vec<RobotFactoryState> {
        let mut result = vec![];
        if state.elapsed_time >= time_limit {
            return result;
        }
        if self.get_max_ore() > state.ore_robots {
            let result_turns = self.time_until_ore(state);
            if result_turns + state.elapsed_time + 1 < time_limit {
                let mut result_state = *state;
                self.step_turns(&mut result_state, result_turns);
                result_state.step(&Some(RobotFactoryCommand::Ore(self.ore_cost)));
                result.push(result_state);
            }
        }
        if self.get_max_clay() > state.clay_robots {
            let result_turns = self.time_until_clay(state);
            if result_turns + state.elapsed_time + 1 < time_limit {
                let mut result_state = *state;
                self.step_turns(&mut result_state, result_turns);
                result_state.step(&Some(RobotFactoryCommand::Clay(self.clay_cost)));
                result.push(result_state);
            }
        }
        if self.get_max_obsidian() > state.obsidian_robots {
            if let Some(result_turns) = self.time_until_obsidian(state) {
                if result_turns + state.elapsed_time + 1 < time_limit {
                    let mut result_state = *state;
                    self.step_turns(&mut result_state, result_turns);
                    result_state.step(&Some(RobotFactoryCommand::Obsidian(
                        self.obsidian_cost.0,
                        self.obsidian_cost.1,
                    )));
                    result.push(result_state);
                }
            }
        }
        if let Some(result_turns) = self.time_until_geode(state) {
            if result_turns + state.elapsed_time + 1 < time_limit {
                let mut result_state = *state;
                self.step_turns(&mut result_state, result_turns);
                result_state.step(&Some(RobotFactoryCommand::Geode(
                    self.geode_cost.0,
                    self.geode_cost.1,
                )));
                result.push(result_state);
            }
        }
        // If we can't build any more robots within the time limit, run out the clock with no-op steps.
        if result.is_empty() && state.elapsed_time + 1 <= time_limit {
            let mut result_state = *state;
            for _ in state.elapsed_time..time_limit {
                result_state.step(&None);
            }
            result.push(result_state);
        }

        result
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    fn get_max_ore(&self) -> usize {
        self.ore_cost
            .max(self.clay_cost)
            .max(self.obsidian_cost.0)
            .max(self.geode_cost.0)
    }

    fn get_max_clay(&self) -> usize {
        self.obsidian_cost.1
    }

    fn get_max_obsidian(&self) -> usize {
        self.geode_cost.1
    }

    fn time_until_ore(&self, state: &RobotFactoryState) -> usize {
        if state.ore >= self.ore_cost {
            0
        } else {
            ((self.ore_cost - state.ore) + state.ore_robots - 1) / state.ore_robots
        }
    }

    fn time_until_clay(&self, state: &RobotFactoryState) -> usize {
        if state.ore >= self.clay_cost {
            0
        } else {
            ((self.clay_cost - state.ore) + state.ore_robots - 1) / state.ore_robots
        }
    }

    fn time_until_obsidian(&self, state: &RobotFactoryState) -> Option<usize> {
        if state.clay_robots == 0 {
            None
        } else if state.ore >= self.obsidian_cost.0 && state.clay >= self.obsidian_cost.1 {
            Some(0)
        } else {
            let ore_time = self
                .obsidian_cost
                .0
                .checked_sub(state.ore)
                .map(|div| (div + state.ore_robots - 1) / state.ore_robots)
                .unwrap_or(0);
            let clay_time = self
                .obsidian_cost
                .1
                .checked_sub(state.clay)
                .map(|div| (div + state.clay_robots - 1) / state.clay_robots)
                .unwrap_or(0);
            Some(ore_time.max(clay_time))
        }
    }

    fn time_until_geode(&self, state: &RobotFactoryState) -> Option<usize> {
        if state.obsidian_robots == 0 {
            None
        } else if state.ore >= self.geode_cost.0 && state.obsidian >= self.geode_cost.1 {
            Some(0)
        } else {
            let ore_time = self
                .geode_cost
                .0
                .checked_sub(state.ore)
                .map(|div| (div + state.ore_robots - 1) / state.ore_robots)
                .unwrap_or(0);
            let obs_time = self
                .geode_cost
                .1
                .checked_sub(state.obsidian)
                .map(|div| (div + state.obsidian_robots - 1) / state.obsidian_robots)
                .unwrap_or(0);
            Some(ore_time.max(obs_time))
        }
    }

    fn step_turns(&self, state: &mut RobotFactoryState, turns: usize) {
        for _ in 0..turns {
            state.step(&None);
        }
    }
}

impl From<&str> for RobotBlueprint {
    fn from(input: &str) -> Self {
        let mut s1 = input.split(':');
        let id = s1
            .next()
            .unwrap()
            .split(' ')
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let mut s2 = s1.next().unwrap().split('.').map(|s| {
            s.split(' ')
                .map(|w| w.parse::<usize>())
                .filter_map(|r| match r.is_ok() {
                    true => Some(r.unwrap()),
                    false => None,
                })
        });
        let ore = s2.next().unwrap().next().unwrap();
        let clay = s2.next().unwrap().next().unwrap();
        let mut s_o = s2.next().unwrap();
        let obsidian = (s_o.next().unwrap(), s_o.next().unwrap());
        let mut s_g = s2.next().unwrap();
        let geode = (s_g.next().unwrap(), s_g.next().unwrap());
        RobotBlueprint {
            id,
            ore_cost: ore,
            clay_cost: clay,
            obsidian_cost: obsidian,
            geode_cost: geode,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::robot_blueprint::RobotBlueprint;

    #[test]
    fn from() {
        let input = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        let bp = RobotBlueprint::from(input);
        assert_eq!(
            bp,
            RobotBlueprint {
                id: 2,
                ore_cost: 2,
                clay_cost: 3,
                obsidian_cost: (3, 8),
                geode_cost: (3, 12)
            }
        );
    }
}
