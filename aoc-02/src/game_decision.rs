use crate::game_choice::GameChoice;
use crate::game_outcome::GameOutcome;

pub struct GameDecision {
    value: char,
}

impl GameDecision {
    pub fn to_choice(&self) -> GameChoice {
        match self.value {
            'A' => GameChoice::Rock,
            'B' => GameChoice::Paper,
            'C' => GameChoice::Scissors,
            'X' => GameChoice::Rock,
            'Y' => GameChoice::Paper,
            'Z' => GameChoice::Scissors,
            _ => panic!(),
        }
    }

    pub fn to_outcome(&self) -> GameOutcome {
        match self.value {
            'X' => GameOutcome::Lose,
            'Y' => GameOutcome::Draw,
            'Z' => GameOutcome::Win,
            _ => panic!(),
        }
    }
}

impl From<char> for GameDecision {
    fn from(input: char) -> Self {
        GameDecision { value: input }
    }
}
