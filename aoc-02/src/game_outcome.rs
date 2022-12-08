use crate::game_choice::GameChoice;

const CHOICES: [GameChoice; 3] = [GameChoice::Rock, GameChoice::Paper, GameChoice::Scissors];

#[derive(PartialEq)]
pub enum GameOutcome {
    Win,
    Lose,
    Draw,
}

impl GameOutcome {
    pub fn score(&self) -> u8 {
        match self {
            GameOutcome::Win => 6,
            GameOutcome::Draw => 3,
            GameOutcome::Lose => 0,
        }
    }

    pub fn find_choice_for(&self, other: &GameChoice) -> GameChoice {
        for choice in CHOICES {
            if &choice.shoot(other) == self {
                return choice;
            }
        }
        panic!("No valid choice found!")
    }
}
