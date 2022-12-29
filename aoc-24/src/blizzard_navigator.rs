use crate::blizzard_map::{BlizzardMap, BlizzardMapPoint};
use crate::blizzard_navigator_state::BlizzardNavigatorState;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct BlizzardNavigator {
    pub start: BlizzardMapPoint,
    pub end: BlizzardMapPoint,
    maps: HashMap<usize, BlizzardMap>,
}

impl BlizzardNavigator {
    pub fn find_shortest_path(self) -> (usize, usize) {
        let start = self.start.clone();
        let end = self.end.clone();
        let rc = Rc::new(RefCell::new(self));
        let initial = BlizzardNavigatorState::new(rc.clone(), &start);
        let first_step = initial.find_path(&end);
        let first_step_count = first_step.step;
        let second_step = first_step.find_path(&start);
        let third_step = second_step.find_path(&end);
        (first_step_count, third_step.step)
    }

    pub fn get_map(&mut self, step: &usize) -> &BlizzardMap {
        if !self.maps.contains_key(step) {
            let prev = self.get_map(&(step - 1)).clone();
            self.maps.insert(*step, prev.get_next());
        }
        self.maps.get(step).unwrap()
    }
}

impl From<&str> for BlizzardNavigator {
    fn from(input: &str) -> Self {
        let start = (1, 0);
        let end = (
            input.lines().next().unwrap().chars().count() - 2,
            input.lines().count() - 1,
        );
        let maps = HashMap::from([(0, BlizzardMap::from(input))]);

        BlizzardNavigator { start, end, maps }
    }
}

#[cfg(test)]
mod tests {
    use crate::blizzard_navigator::BlizzardNavigator;

    fn get_input() -> BlizzardNavigator {
        r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"
            .into()
    }

    #[test]
    fn max() {
        let mut map = get_input();
        assert_eq!(map.start, (1, 0));
        assert_eq!(map.end, (6, 5));
        let map0 = map.get_map(&0);
        assert_eq!(map0.max_x, 7);
        assert_eq!(map0.max_y, 5);
    }

    #[test]
    fn example() {
        let map = get_input();
        let shortest_path = map.find_shortest_path();

        assert_eq!(shortest_path.0, 18);
        assert_eq!(shortest_path.1, 54);
    }
}
