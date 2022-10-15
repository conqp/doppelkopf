use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct Player {
    id: u64,
    name: String,
    points: i64,
}

impl Player {
    pub fn new(id: u64, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            points: 0,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn points(&self) -> i64 {
        self.points
    }

    pub fn add(&mut self, points: i64) {
        self.points += points
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{} {}: {}", self.id, self.name, self.points)
    }
}
