use cardlib::Suit;

#[derive(Debug)]
pub enum SoloType {
    Regular,
    SilentWedding,
    Fleshless,
    Suit(Suit),
}
