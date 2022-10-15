use crate::rule_set::RuleSet;
use cardlib::{Card, Face, Suit};
use std::cmp::Ordering;

pub const TRUMPS: [Card; 13] = [
    Card::new(Suit::Diamonds, Face::Nine),
    Card::new(Suit::Diamonds, Face::King),
    Card::new(Suit::Diamonds, Face::Ten),
    Card::new(Suit::Diamonds, Face::Ace),
    Card::new(Suit::Diamonds, Face::Jack),
    Card::new(Suit::Hearts, Face::Jack),
    Card::new(Suit::Spades, Face::Jack),
    Card::new(Suit::Clubs, Face::Jack),
    Card::new(Suit::Diamonds, Face::Queen),
    Card::new(Suit::Hearts, Face::Queen),
    Card::new(Suit::Spades, Face::Queen),
    Card::new(Suit::Clubs, Face::Queen),
    Card::new(Suit::Hearts, Face::Ten),
];
const FACES: [Face; 4] = [Face::Nine, Face::King, Face::Ten, Face::Ace];

#[derive(Debug)]
pub struct StandardGame {
    trumps: [Card; 13],
    faces: [Face; 4],
}

impl StandardGame {
    pub fn new() -> Self {
        Self {
            trumps: TRUMPS,
            faces: FACES,
        }
    }
}

impl RuleSet for StandardGame {
    fn compare(&self, played: &Card, lying: &Card) -> Ordering {
        match self.trumps.iter().position(|card| card == played) {
            Some(played) => match self.trumps.iter().position(|card| card == lying) {
                Some(lying) => played.cmp(&lying),
                None => Ordering::Greater,
            },
            None => match self.trumps.iter().position(|card| card == lying) {
                Some(_) => Ordering::Less,
                None => {
                    if played.suit() == lying.suit() {
                        self.faces
                            .iter()
                            .position(|face| face == played.face())
                            .unwrap_or(0)
                            .cmp(
                                &self
                                    .faces
                                    .iter()
                                    .position(|face| face == lying.face())
                                    .unwrap_or(0),
                            )
                    } else {
                        Ordering::Less
                    }
                }
            },
        }
    }

    fn is_trump(&self, card: &Card) -> bool {
        self.trumps.iter().any(|trump| card == trump)
    }

    fn serves(&self, hand: Vec<Card>, played: &Card, lying: &Card) -> bool {
        if self.is_trump(lying) {
            if self.is_trump(played) {
                true
            } else {
                hand.iter().all(|card| !self.is_trump(card))
            }
        } else if !self.is_trump(played) && played.suit() == lying.suit() {
            true
        } else {
            hand.iter().all(|card| card.suit() != lying.suit())
        }
    }
}
