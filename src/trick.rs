use crate::player::Player;
use crate::round_player::RoundPlayer;
use crate::rule_set::RuleSet;
use crate::turn_result::TurnResult;
use cardlib::Card;
use std::cmp::Ordering;

pub struct Trick {
    players: [RoundPlayer; 4],
    rules: Box<dyn RuleSet>,
    cards: Vec<Card>,
    index: usize,
    owner: Option<usize>,
}

impl Trick {
    pub fn new(players: [RoundPlayer; 4], rules: impl RuleSet + 'static) -> Self {
        Self {
            players,
            rules: Box::new(rules),
            cards: Vec::new(),
            index: 0,
            owner: None,
        }
    }

    pub fn play(&mut self, player: &Player, card: Card) -> TurnResult {
        if self.current().id() != player.id() {
            TurnResult::NotYourTurn
        } else {
            match self.cards.clone().last() {
                Some(lying) => {
                    if self.rules.serves(self.current().hand(), &card, lying) {
                        self.cards.push(card);
                        if self.rules.compare(&card, lying) == Ordering::Greater {
                            self.owner = Some(self.index);
                        }
                        TurnResult::Ok
                    } else {
                        TurnResult::InvalidServe
                    }
                }
                None => {
                    self.cards.push(card);
                    self.owner = Some(self.index);
                    TurnResult::Ok
                }
            }
        }
    }

    fn current(&self) -> &RoundPlayer {
        &self.players[self.index]
    }

    fn next(&mut self) {
        if self.index >= self.players.len() {
            self.index = 0;
        } else {
            self.index += 1;
        }
    }
}
