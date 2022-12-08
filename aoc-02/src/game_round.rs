use crate::game_choice::GameChoice;
use crate::game_decision::GameDecision;

pub struct GameRound {
    player: GameDecision,
    other: GameChoice,
}

impl GameRound {
    pub fn new(player: GameDecision, other: GameChoice) -> Self {
        GameRound { player, other }
    }

    pub fn score_as_choice(&self) -> u8 {
        let player_choice = self.player.to_choice();
        player_choice.shoot(&self.other).score() + player_choice.score()
    }

    pub fn score_as_outcome(&self) -> u8 {
        let outcome = self.player.to_outcome();
        let player_choice = outcome.find_choice_for(&self.other);
        outcome.score() + player_choice.score()
    }
}
