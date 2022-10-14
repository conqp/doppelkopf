use crate::player::Player;
use cardlib::Card;

#[derive(Debug)]
pub struct RoundPlayer<'a> {
    player: &'a mut Player,
    hand: Vec<Card>,
}

impl<'a> RoundPlayer<'a> {
    fn new(player: &'a mut Player) -> Self {
        Self {
            player,
            hand: Vec::new(),
        }
    }
}
