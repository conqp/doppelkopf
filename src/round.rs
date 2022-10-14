use crate::game_type::GameType;
use crate::player::Player;
use crate::round_player::RoundPlayer;

#[derive(Debug)]
pub struct Round {
    players: [RoundPlayer; 4],
    game_type: Option<GameType>,
}

impl Round {
    pub fn new(players: [Player; 4]) -> Self {
        Self {
            players: [
                RoundPlayer::new(players[0].id()),
                RoundPlayer::new(players[1].id()),
                RoundPlayer::new(players[2].id()),
                RoundPlayer::new(players[3].id()),
            ],
            game_type: None,
        }
    }

    pub fn players(&self) -> &[RoundPlayer; 4] {
        &self.players
    }

    pub fn game_type(&self) -> &Option<GameType> {
        &self.game_type
    }
}
