use std::fmt::{Display, Formatter};

pub enum RobotFactoryCommand {
    Ore(usize),
    Clay(usize),
    Obsidian(usize, usize),
    Geode(usize, usize),
}

impl Display for RobotFactoryCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RobotFactoryCommand::Ore(_) => f.write_str("ore robot"),
            RobotFactoryCommand::Clay(_) => f.write_str("clay robot"),
            RobotFactoryCommand::Obsidian(_, _) => f.write_str("obsidian robot"),
            RobotFactoryCommand::Geode(_, _) => f.write_str("geode robot"),
        }
    }
}
