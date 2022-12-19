use crate::valve_location::ValveLocation;
use crate::valve_map::ValveMap;
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::iter::from_fn;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

const THREADS: usize = 10;

pub struct ValveMapStateContainer {
    map: Arc<ValveMap>,
}

impl ValveMapStateContainer {
    pub fn new(map: ValveMap) -> Self {
        ValveMapStateContainer { map: Arc::new(map) }
    }

    pub fn find_max_pressure(&self, has_elephant: bool) -> usize {
        let i = ValveMapState::new(Arc::clone(&self.map), has_elephant);
        let mut initial: Vec<ValveMapState> = i.step().collect();
        let total = initial.len();
        println!("i {}", initial.len());

        let mut handles: VecDeque<JoinHandle<usize>> = VecDeque::with_capacity(THREADS);
        for _ in 0..THREADS.min(initial.len()) {
            handles.push_back(ValveMapStateContainer::make_worker(initial.pop().unwrap()));
        }

        let mut completed = 0usize;
        let mut max = 0usize;
        while let Some(h) = handles.pop_front() {
            if h.is_finished() {
                let r = h.join().unwrap();
                completed += 1;
                println!("[{}/{}] Chunk finished with: {}", completed, total, r);
                if r > max {
                    max = r;
                }
                if let Some(n) = initial.pop() {
                    handles.push_back(ValveMapStateContainer::make_worker(n))
                }
            } else {
                handles.push_back(h);
            }
        }

        max
    }

    fn make_worker(to_process: ValveMapState) -> JoinHandle<usize> {
        let max = match to_process.has_elephant {
            true => 25,
            false => 29,
        };
        thread::spawn(move || {
            let mut iter: Box<dyn Iterator<Item = ValveMapState>> =
                Box::new([to_process].into_iter());
            for _ in 0..max {
                iter = Box::new(iter.flat_map(|s| s.step()));
            }
            iter.map(|s| s.get_score()).max().unwrap()
        })
    }
}

#[derive(Debug, Clone)]
struct ValveMapState {
    map: Arc<ValveMap>,
    pub minutes_remaining: u8,
    person_target: Option<VecDeque<ValveLocation>>,
    person_location: ValveLocation,
    elephant_target: Option<VecDeque<ValveLocation>>,
    elephant_location: ValveLocation,
    activated: HashSet<ValveMapActivation>,
    has_elephant: bool,
}

impl ValveMapState {
    fn new(map: Arc<ValveMap>, has_elephant: bool) -> Self {
        let minutes_remaining = match has_elephant {
            true => 26,
            false => 30,
        };
        ValveMapState {
            map,
            minutes_remaining,
            person_target: None,
            person_location: ValveLocation::from("AA"),
            elephant_target: None,
            elephant_location: ValveLocation::from("AA"),
            activated: HashSet::new(),
            has_elephant,
        }
    }

    pub fn get_score(&self) -> usize {
        let mut score = 0usize;
        for activation in self.activated.iter() {
            let valve = self.map.valves.get(&activation.loc).unwrap();
            score += valve.flow * activation.minutes_remaining as usize;
        }

        score
    }

    pub fn step(mut self) -> impl Iterator<Item = ValveMapState> {
        self.minutes_remaining -= 1;

        let person_needs_target = self.person_target.is_none();
        let mut is_person_complete = false;
        let mut person_targets = if person_needs_target {
            let r = self.get_valid_targets(&self.person_location);
            if r.len() == 0 {
                None
            } else {
                Some(r)
            }
        } else {
            None
        };

        let person_iter = from_fn(move || {
            if person_targets.is_none() {
                match is_person_complete {
                    true => None,
                    false => {
                        is_person_complete = true;
                        self.step_person();
                        Some(self.to_owned())
                    }
                }
            } else {
                person_targets.as_mut().unwrap().pop().map(|l| {
                    let mut next = self.clone();
                    let target = next.map.get_path(&next.person_location, &l);
                    next.person_target = Some(VecDeque::from(target));
                    next.step_person();
                    next
                })
            }
        });

        let elephant_iter = person_iter.flat_map(|mut p| {
            let mut is_elephant_complete = false;
            let elephant_needs_target = p.elephant_target.is_none() && p.has_elephant;
            let mut elephant_targets = if elephant_needs_target {
                let r = p.get_valid_targets(&p.elephant_location);
                if r.len() == 0 {
                    None
                } else {
                    Some(r)
                }
            } else {
                None
            };

            from_fn(move || {
                if elephant_targets.is_none() {
                    match is_elephant_complete {
                        true => None,
                        false => {
                            is_elephant_complete = true;
                            p.step_elephant();
                            Some(p.to_owned())
                        }
                    }
                } else {
                    elephant_targets.as_mut().unwrap().pop().map(|l| {
                        let mut next = p.clone();
                        let target = next.map.get_path(&next.elephant_location, &l);
                        next.elephant_target = Some(VecDeque::from(target));
                        next.step_elephant();
                        next
                    })
                }
            })
        });

        elephant_iter
    }

    fn step_person(&mut self) {
        if self.person_target.is_none() {
            return;
        }
        if let Some(next) = self.person_target.as_mut().unwrap().pop_front() {
            self.person_location = next;
        } else {
            self.activated.insert(ValveMapActivation::new(
                self.person_location,
                self.minutes_remaining,
            ));
            self.person_target = None;
        }
    }

    fn step_elephant(&mut self) {
        if self.has_elephant && self.elephant_target.is_some() {
            if let Some(next) = self.elephant_target.as_mut().unwrap().pop_front() {
                self.elephant_location = next;
            } else {
                self.activated.insert(ValveMapActivation::new(
                    self.elephant_location,
                    self.minutes_remaining,
                ));
                self.elephant_target = None;
            }
        }
    }

    fn get_valid_targets(&self, current_loc: &ValveLocation) -> Vec<ValveLocation> {
        let mut result = self
            .map
            .valves
            .values()
            .filter(|v| v.flow > 0)
            .map(|v| v.loc)
            .filter(|v| !self.activated.contains(&ValveMapActivation::new(*v, 0)))
            .filter(|v| {
                self.person_target
                    .as_ref()
                    .map(|t| t.back().map(|l| v != l).unwrap_or(true))
                    .unwrap_or(true)
                    && self
                        .elephant_target
                        .as_ref()
                        .map(|t| t.back().map(|l| v != l).unwrap_or(true))
                        .unwrap_or(true)
            })
            .filter(|v| {
                let path = self.map.get_path(current_loc, v);
                self.minutes_remaining as usize > path.len() + 1
            })
            .map(|v| {
                let a_v = self.map.valves.get(&v).unwrap();
                let a_path = self.map.get_path(current_loc, &v);
                let score = a_v.flow * (self.minutes_remaining as usize - a_path.len() - 1);
                (v, score)
            })
            .collect::<Vec<(ValveLocation, usize)>>();

        result.sort_by(|a, b| b.1.cmp(&a.1));
        if result.len() == 0 {
            return vec![];
        }
        let best = result.first().unwrap().1;
        result
            .into_iter()
            .filter(|(_, s)| s * 3 > best)
            .map(|(v, _)| v)
            .collect()
    }
}

unsafe impl Send for ValveMapState {}

impl<'a> PartialEq for ValveMapState {
    fn eq(&self, other: &Self) -> bool {
        self.minutes_remaining.eq(&other.minutes_remaining)
            && self.activated.eq(&other.activated)
            && self.person_location.eq(&other.person_location)
    }
}

impl<'a> Eq for ValveMapState {}

impl<'a> PartialOrd for ValveMapState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for ValveMapState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_score().cmp(&other.get_score())
    }
}

#[derive(Debug, Copy, Clone)]
struct ValveMapActivation {
    loc: ValveLocation,
    minutes_remaining: u8,
}

impl ValveMapActivation {
    pub fn new(loc: ValveLocation, minutes_remaining: u8) -> Self {
        ValveMapActivation {
            loc,
            minutes_remaining,
        }
    }
}

impl PartialEq for ValveMapActivation {
    fn eq(&self, other: &Self) -> bool {
        self.loc.eq(&other.loc)
    }
}

impl Eq for ValveMapActivation {}

impl Hash for ValveMapActivation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.loc.hash(state);
    }
}
