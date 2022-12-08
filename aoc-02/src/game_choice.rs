use crate::game_outcome::GameOutcome;

#[derive(Copy, Clone, PartialEq)]
pub enum GameChoice {
    Rock,
    Paper,
    Scissors,
}

impl GameChoice {
    pub fn score(&self) -> u8 {
        match self {
            GameChoice::Rock => 1,
            GameChoice::Paper => 2,
            GameChoice::Scissors => 3,
        }
    }

    pub fn shoot(&self, other: &GameChoice) -> GameOutcome {
        match self {
            GameChoice::Rock => match other {
                GameChoice::Paper => GameOutcome::Lose,
                GameChoice::Scissors => GameOutcome::Win,
                GameChoice::Rock => GameOutcome::Draw,
            },
            GameChoice::Paper => match other {
                GameChoice::Scissors => GameOutcome::Lose,
                GameChoice::Rock => GameOutcome::Win,
                GameChoice::Paper => GameOutcome::Draw,
            },
            GameChoice::Scissors => match other {
                GameChoice::Rock => GameOutcome::Lose,
                GameChoice::Paper => GameOutcome::Win,
                GameChoice::Scissors => GameOutcome::Draw,
            },
        }
    }
}
