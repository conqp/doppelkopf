use crate::player::Player;
use crate::player_state::PlayerState;
use cardlib::Card;

#[derive(Debug)]
pub struct RoundPlayer<'a> {
    player: &'a mut Player,
    hand: Vec<Card>,
    state: Option<PlayerState>,
}

impl<'a> RoundPlayer<'a> {
    fn new(player: &'a mut Player, hand: Vec<Card>) -> Self {
        Self {
            player,
            hand,
            state: None,
        }
    }
}
