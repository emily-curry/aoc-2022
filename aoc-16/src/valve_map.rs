use crate::valve::Valve;
use crate::valve_location::ValveLocation;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::str::Lines;

#[derive(Debug)]
pub struct ValveMap {
    pub valves: HashMap<ValveLocation, Valve>,
}

impl ValveMap {
    pub fn get_path(&self, current: &ValveLocation, target: &ValveLocation) -> Vec<ValveLocation> {
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::from([ValveMapStep::new(0, *target)]);

        while let Some(step) = queue.pop() {
            let valve = self.valves.get(&step.loc).unwrap();
            for next_loc in valve.tunnels.iter() {
                let next_step = ValveMapStep::new(step.step + 1, *next_loc);
                if !visited.contains(&next_step) {
                    visited.insert(step);
                    queue.push(next_step);
                }
            }
            if valve.tunnels.contains(current) {
                queue.push(ValveMapStep::new(step.step + 1, *current));
                break;
            }
        }

        let initial_step = queue.iter().find(|s| &s.loc == current).unwrap().to_owned();
        let mut result = Vec::from([initial_step]);
        while let Some(_) = result.last().map(|s| s.step.checked_sub(1)).unwrap() {
            let valve = self.valves.get(&result.last().unwrap().loc).unwrap();
            let next = visited
                .iter()
                .filter(|s| valve.tunnels.contains(&s.loc))
                .max()
                .unwrap();
            result.push(*next);
        }
        result.iter().skip(1).map(|x| x.loc.to_owned()).collect()
    }
}

impl From<Lines<'_>> for ValveMap {
    fn from(input: Lines<'_>) -> Self {
        let valves = input.map(Valve::from).map(|v| (v.loc, v)).collect();
        ValveMap { valves }
    }
}

#[derive(Debug, Clone, Copy)]
struct ValveMapStep {
    step: usize,
    loc: ValveLocation,
}

impl ValveMapStep {
    pub fn new(step: usize, loc: ValveLocation) -> Self {
        ValveMapStep { step, loc }
    }
}

impl PartialEq for ValveMapStep {
    fn eq(&self, other: &Self) -> bool {
        self.loc.eq(&other.loc)
    }
}

impl Eq for ValveMapStep {}

impl PartialOrd for ValveMapStep {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ValveMapStep {
    fn cmp(&self, other: &Self) -> Ordering {
        other.step.cmp(&self.step)
    }
}

impl Hash for ValveMapStep {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.loc.hash(state);
    }
}
