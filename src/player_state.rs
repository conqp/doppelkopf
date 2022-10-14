use crate::reservation::Reservation;

#[derive(Debug)]
pub enum PlayerState {
    Healthy,
    Reservation(Reservation),
}
