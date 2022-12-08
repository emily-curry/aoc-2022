use std::str::Lines;

use crate::game_decision::GameDecision;
use crate::game_round::GameRound;

pub struct GameTournament {
    rounds: Vec<GameRound>,
}

impl GameTournament {
    pub fn sum_scores_as_choice(&self) -> u64 {
        self.rounds
            .iter()
            .fold(0u64, |acc, val| acc + val.score_as_choice() as u64)
    }

    pub fn sum_scores_as_outcome(&self) -> u64 {
        self.rounds
            .iter()
            .fold(0u64, |acc, val| acc + val.score_as_outcome() as u64)
    }
}

impl From<Lines<'_>> for GameTournament {
    fn from(input: Lines<'_>) -> Self {
        let rounds = input
            .map(|line| {
                let mut split = line
                    .split(' ')
                    .map(|x| GameDecision::from(x.chars().next().unwrap()));
                let other = split.next().unwrap().to_choice();
                let player = split.next().unwrap();
                GameRound::new(player, other)
            })
            .collect();
        GameTournament { rounds }
    }
}
