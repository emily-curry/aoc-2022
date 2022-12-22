use crate::robot_factory_command::RobotFactoryCommand;

#[derive(Debug, Copy, Clone)]
pub struct RobotFactoryState {
    pub elapsed_time: usize,
    pub ore_robots: usize,
    pub clay_robots: usize,
    pub obsidian_robots: usize,
    pub geode_robots: usize,
    pub ore: usize,
    pub clay: usize,
    pub obsidian: usize,
    pub geode: usize,
}

impl RobotFactoryState {
    pub fn step(&mut self, command: &Option<RobotFactoryCommand>) {
        self.elapsed_time += 1;
        if let Some(cmd) = &command {
            match cmd {
                RobotFactoryCommand::Ore(ore) => self.ore -= ore,
                RobotFactoryCommand::Clay(ore) => self.ore -= ore,
                RobotFactoryCommand::Obsidian(ore, clay) => {
                    self.ore -= ore;
                    self.clay -= clay;
                }
                RobotFactoryCommand::Geode(ore, obsidian) => {
                    self.ore -= ore;
                    self.obsidian -= obsidian;
                }
            }
        }
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;

        if let Some(cmd) = &command {
            match cmd {
                RobotFactoryCommand::Ore(_) => self.ore_robots += 1,
                RobotFactoryCommand::Clay(_) => self.clay_robots += 1,
                RobotFactoryCommand::Obsidian(_, _) => self.obsidian_robots += 1,
                RobotFactoryCommand::Geode(_, _) => self.geode_robots += 1,
            };
        }
    }

    /// Returns whether self will always outproduce other, given the most conservative strategy for self and the most optimistic strategy for other.
    pub fn will_always_outproduce(&self, other: &Self, time_limit: usize) -> bool {
        // Number of geodes that will be produced if self just stops doing anything at all.
        let self_geodes =
            (self.elapsed_time..time_limit).fold(self.geode, |acc, _| acc + self.geode_robots);
        // Number of geodes that will be produced if other creates a geode robot every step until the end.
        let other_geodes = (0..(time_limit - other.elapsed_time + 1))
            .fold(other.geode, |acc, i| acc + (other.geode_robots + i));
        self_geodes >= other_geodes
    }
}

impl Default for RobotFactoryState {
    fn default() -> Self {
        RobotFactoryState {
            elapsed_time: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}
