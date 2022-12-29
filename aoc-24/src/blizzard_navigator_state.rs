use crate::blizzard_map::BlizzardMapPoint;
use crate::blizzard_navigator::BlizzardNavigator;
use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct BlizzardNavigatorState {
    pub position: BlizzardMapPoint,
    nav: Rc<RefCell<BlizzardNavigator>>,
    pub step: usize,
}

impl BlizzardNavigatorState {
    pub fn new(nav: Rc<RefCell<BlizzardNavigator>>, from: &BlizzardMapPoint) -> Self {
        BlizzardNavigatorState {
            position: *from,
            nav,
            step: 0,
        }
    }

    pub fn find_path(self, target: &BlizzardMapPoint) -> Self {
        let mut states = HashSet::from([self]);
        loop {
            let prev_states = states.clone();
            states.clear();
            for state in prev_states {
                if state.position == *target {
                    return state;
                }
                states.extend(state.get_next_states());
            }
        }
    }

    fn get_next_states(&self) -> Vec<Self> {
        let mut possible_positions: Vec<BlizzardNavigatorState> = Vec::new();
        let next_step = self.step + 1;
        let mut nav = self.nav.borrow_mut();
        let next_map = nav.get_map(&next_step);
        if !next_map.contains(&self.position) {
            possible_positions.push(self.clone_with_position(self.position));
        }
        if self.position.1 > 0 {
            let up = (self.position.0, self.position.1 - 1);
            if up.1 > 0 && !next_map.contains(&up) {
                possible_positions.push(self.clone_with_position(up));
            }
            if self.position.1 < next_map.max_y {
                let left = (self.position.0 - 1, self.position.1);
                if left.0 > 0 && !next_map.contains(&left) {
                    possible_positions.push(self.clone_with_position(left));
                }
                let right = (self.position.0 + 1, self.position.1);
                if right.0 < next_map.max_x && !next_map.contains(&right) {
                    possible_positions.push(self.clone_with_position(right));
                }
            }
        }
        let down = (self.position.0, self.position.1 + 1);
        if down.1 < next_map.max_y && !next_map.contains(&down) {
            possible_positions.push(self.clone_with_position(down));
        }
        if self.position == (next_map.max_x - 1, next_map.max_y - 1) {
            possible_positions.push(self.clone_with_position((next_map.max_x - 1, next_map.max_y)));
        }
        if self.position == (1, 1) {
            possible_positions.push(self.clone_with_position((1, 0)));
        }

        possible_positions
    }

    fn clone_with_position(&self, position: BlizzardMapPoint) -> Self {
        BlizzardNavigatorState {
            position,
            nav: Rc::clone(&self.nav),
            step: self.step + 1,
        }
    }
}

impl PartialEq for BlizzardNavigatorState {
    fn eq(&self, other: &Self) -> bool {
        self.position.eq(&other.position) && self.step.eq(&other.step)
    }
}

impl Eq for BlizzardNavigatorState {}

impl Hash for BlizzardNavigatorState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.step.hash(state);
    }
}
