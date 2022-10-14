use crate::solo_type::SoloType;

#[derive(Debug)]
pub enum Reservation {
    Wedding,
    Solo(SoloType),
}
