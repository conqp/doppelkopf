use crate::game_type::GameType;
use crate::round_player::RoundPlayer;

#[derive(Debug)]
pub struct Round<'a> {
    game_type: Option<GameType>,
    players: [RoundPlayer<'a>; 4],
}
