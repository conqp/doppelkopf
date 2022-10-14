use crate::player_state::PlayerState;
use crate::team::Team;
use cardlib::Card;

#[derive(Debug)]
pub struct RoundPlayer {
    id: u64,
    hand: Option<Vec<Card>>,
    state: Option<PlayerState>,
    team: Option<Team>,
}

impl RoundPlayer {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            hand: None,
            state: None,
            team: None,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn deal(&mut self, hand: Vec<Card>) {
        self.hand = Some(hand)
    }

    pub fn state(&self) -> &Option<PlayerState> {
        &self.state
    }

    pub fn team(&self) -> &Option<Team> {
        &self.team
    }

    pub fn is_ready(&self) -> bool {
        self.state.is_some()
    }

    pub fn teamed_up(&self) -> bool {
        self.team.is_some()
    }
}
