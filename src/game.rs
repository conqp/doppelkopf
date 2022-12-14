use crate::player::Player;
use crate::round::Round;

#[derive(Debug)]
pub struct Game {
    players: [Player; 4],
    rounds: Vec<Round>,
}
