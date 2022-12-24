use std::collections::VecDeque;
use std::str::Lines;

const DECRYPTION_KEY: isize = 811589153;

#[derive(Debug)]
pub struct Decrypter {
    numbers: VecDeque<(usize, isize)>,
}

impl Decrypter {
    pub fn get_coords(&self, mix_count: usize) -> (isize, isize, isize) {
        let mut mixed = self.numbers.clone();
        for _ in 0..mix_count {
            self.mix(&mut mixed);
        }
        let zero_idx = mixed.iter().position(|m| m.1 == 0).unwrap();
        let first_idx = (zero_idx + 1000) % mixed.len();
        let second_idx = (zero_idx + 2000) % mixed.len();
        let third_idx = (zero_idx + 3000) % mixed.len();
        (mixed[first_idx].1, mixed[second_idx].1, mixed[third_idx].1)
    }

    pub fn apply_key(&mut self) {
        for (_, v) in self.numbers.iter_mut() {
            *v *= DECRYPTION_KEY;
        }
    }

    fn mix(&self, mixed: &mut VecDeque<(usize, isize)>) {
        for (key, n) in self.numbers.iter() {
            let ci = mixed.iter().position(|m| m.0 == *key).unwrap() as isize;
            mixed.remove(ci as usize);
            let next_i = (ci + n).rem_euclid(mixed.len() as isize);
            mixed.insert(next_i as usize, (*key, *n));
        }
    }
}

impl From<Lines<'_>> for Decrypter {
    fn from(input: Lines<'_>) -> Self {
        let numbers = input.map(|l| l.parse().unwrap()).enumerate().collect();
        Decrypter { numbers }
    }
}

#[cfg(test)]
mod tests {
    use crate::decrypter::Decrypter;

    fn get_input() -> Decrypter {
        let input = r"1
2
-3
3
-2
0
4";
        Decrypter::from(input.lines())
    }

    #[test]
    fn example() {
        let input = get_input();
        let coords = input.get_coords(1);
        assert_eq!(coords, (4, -3, 2));
        assert_eq!(coords.0 + coords.1 + coords.2, 3);
    }

    #[test]
    fn example2() {
        let mut input = get_input();
        input.apply_key();
        let coords = input.get_coords(10);
        assert_eq!(coords, (811589153, 2434767459, -1623178306));
        assert_eq!(coords.0 + coords.1 + coords.2, 1623178306);
    }
}
