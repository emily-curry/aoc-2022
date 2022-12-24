use crate::robot_blueprint::RobotBlueprint;
use crate::robot_factory_state::RobotFactoryState;
use std::collections::VecDeque;

pub struct RobotFactory {
    blueprint: RobotBlueprint,
}

impl RobotFactory {
    pub fn find_geodes_in_steps(&self, time_limit: usize) -> RobotFactoryState {
        let mut max = RobotFactoryState::default();
        let mut queue = VecDeque::from([*&max]);

        while let Some(next) = queue.pop_front() {
            if max.will_always_outproduce(&next, time_limit) {
                continue;
            }
            queue.extend(self.blueprint.find_next_states(&next, time_limit).iter());
            if next.elapsed_time > time_limit {
                panic!(
                    "Elapsed time {} exceeded time limit {}!",
                    next.elapsed_time, time_limit
                );
            }
            if next.geode > max.geode {
                max = next;
            }
        }
        max
    }

    pub fn get_id(&self) -> usize {
        self.blueprint.get_id()
    }
}

impl From<&str> for RobotFactory {
    fn from(input: &str) -> Self {
        let blueprint = RobotBlueprint::from(input);
        RobotFactory { blueprint }
    }
}

#[cfg(test)]
mod tests {
    use crate::robot_factory::RobotFactory;

    fn get_factory_1() -> RobotFactory {
        RobotFactory::from("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.")
    }

    fn get_factory_2() -> RobotFactory {
        RobotFactory::from("Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.")
    }

    #[test]
    fn example1() {
        let f1 = get_factory_1();
        let r = f1.find_geodes_in_steps(24);
        assert_eq!(r.geode * f1.get_id(), 9);

        let f2 = get_factory_2();
        let r = f2.find_geodes_in_steps(24);
        assert_eq!(r.geode * f2.get_id(), 24);
    }

    #[test]
    fn example2_1() {
        let f1 = get_factory_1();
        let r = f1.find_geodes_in_steps(32);
        assert_eq!(r.geode, 56);
    }

    #[test]
    fn example2_2() {
        let f2 = get_factory_2();
        let r = f2.find_geodes_in_steps(32);
        assert_eq!(r.geode, 62);
    }
}
