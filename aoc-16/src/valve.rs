use crate::valve_location::ValveLocation;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Valve {
    pub loc: ValveLocation,
    pub flow: usize,
    pub tunnels: HashSet<ValveLocation>,
}

impl From<&str> for Valve {
    fn from(input: &str) -> Self {
        let re = Regex::new(r"^Valve (\w\w).*rate=(\d+);.*valves? (.*)$")
            .unwrap()
            .captures(input)
            .unwrap();
        let loc = re.get(1).unwrap().as_str().into();
        let flow = re.get(2).unwrap().as_str().parse().unwrap();
        let tunnels = re
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(ValveLocation::from)
            .collect();

        Valve { loc, flow, tunnels }
    }
}
