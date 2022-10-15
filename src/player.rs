use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Eq, PartialEq)]
pub struct Player {
    uuid: Uuid,
    name: String,
    points: i64,
}

impl Player {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name.into(),
            points: 0,
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
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
        write!(f, "#{} {}: {}", self.uuid, self.name, self.points)
    }
}
