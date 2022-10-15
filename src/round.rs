use crate::game_type::GameType;
use crate::player::Player;
use crate::round_player::RoundPlayer;
use cardlib::Card;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
pub struct Round {
    players: [RoundPlayer; 4],
    deck: Vec<Card>,
    trick: Vec<Card>,
    game_type: Option<GameType>,
}

impl Round {
    pub fn new(players: [Player; 4], deck: Vec<Card>) -> Self {
        Self {
            players: [
                RoundPlayer::new(players[0].uuid()),
                RoundPlayer::new(players[1].uuid()),
                RoundPlayer::new(players[2].uuid()),
                RoundPlayer::new(players[3].uuid()),
            ],
            deck,
            trick: Vec::new(),
            game_type: None,
        }
    }

    pub fn players(&self) -> &[RoundPlayer; 4] {
        &self.players
    }

    pub fn deck(&self) -> &Vec<Card> {
        &self.deck
    }

    pub fn trick(&self) -> &Vec<Card> {
        &self.trick
    }

    pub fn game_type(&self) -> &Option<GameType> {
        &self.game_type
    }

    pub fn all_ready(&self) -> bool {
        self.players.iter().all(|player| player.is_ready())
    }

    pub fn all_teamed_up(&self) -> bool {
        self.players.iter().all(|player| player.teamed_up())
    }

    pub fn hand_size(&self) -> usize {
        self.deck.len() / self.players.len()
    }

    pub fn shuffle(&mut self) {
        self.deck.shuffle(&mut thread_rng());
    }

    pub fn deal(&mut self) {
        let hand_size = self.hand_size();

        for (index, player) in self.players.iter_mut().enumerate() {
            let hand = &self.deck[index * hand_size..(index + 1) * hand_size];
            player.deal(hand.into());
        }
    }
}
