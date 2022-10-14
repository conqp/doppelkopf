use crate::bidding::Bidding;
use crate::player_state::PlayerState;
use crate::team::Team;
use cardlib::Card;

#[derive(Debug)]
pub struct RoundPlayer {
    id: u64,
    hand: Option<Vec<Card>>,
    tricks: Vec<[Card; 4]>,
    state: Option<PlayerState>,
    team: Option<Team>,
    called: bool,
    bidding: Option<Bidding>,
}

impl RoundPlayer {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            hand: None,
            tricks: Vec::new(),
            state: None,
            team: None,
            called: false,
            bidding: None,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn hand(&self) -> &Option<Vec<Card>> {
        &self.hand
    }

    pub fn tricks(&self) -> &Vec<[Card; 4]> {
        &self.tricks
    }

    pub fn state(&self) -> &Option<PlayerState> {
        &self.state
    }

    pub fn team(&self) -> &Option<Team> {
        &self.team
    }

    pub fn called(&self) -> bool {
        self.called
    }

    pub fn call(&mut self) {
        self.called = true;
    }

    pub fn bidding(&self) -> &Option<Bidding> {
        &self.bidding
    }

    pub fn bid(&mut self, bidding: Bidding) {
        self.bidding = Some(bidding);
    }

    pub fn is_ready(&self) -> bool {
        self.state.is_some()
    }

    pub fn teamed_up(&self) -> bool {
        self.team.is_some()
    }

    pub fn deal(&mut self, hand: Vec<Card>) {
        self.hand = Some(hand)
    }
}
