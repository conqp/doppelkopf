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

    pub fn running(&self) -> bool {
        self.index < self.players.len()
    }

    pub fn play(&mut self, player: &mut RoundPlayer, card_index: usize) -> TurnResult {
        if self.players[self.index].uuid() != player.uuid() {
            TurnResult::NotYourTurn
        } else {
            match self.play_turn(player, card_index) {
                TurnResult::Ok => {
                    self.index += 1;
                    TurnResult::Ok
                }
                result => result,
            }
        }
    }

    fn play_turn(&mut self, player: &mut RoundPlayer, card_index: usize) -> TurnResult {
        match player.play(card_index) {
            Some(card) => self.play_valid_card(&card),
            None => TurnResult::InvalidServe,
        }
    }

    fn play_valid_card(&mut self, card: &Card) -> TurnResult {
        match self.cards.clone().last() {
            Some(lying) => self.serve_card(card, lying),
            None => self.play_first_card(card),
        }
    }

    fn serve_card(&mut self, card: &Card, lying: &Card) -> TurnResult {
        if self
            .rules
            .serves(self.players[self.index].hand(), card, lying)
        {
            self.cards.push(*card);
            if self.rules.compare(card, lying) == Ordering::Greater {
                self.owner = Some(self.index);
            }
            TurnResult::Ok
        } else {
            TurnResult::InvalidServe
        }
    }

    fn play_first_card(&mut self, card: &Card) -> TurnResult {
        self.cards.push(*card);
        self.owner = Some(self.index);
        TurnResult::Ok
    }
}
