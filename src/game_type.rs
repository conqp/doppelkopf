use crate::solo_type::SoloType;

#[derive(Debug)]
pub enum GameType {
    Regular,
    Wedding,
    Solo(SoloType),
}
