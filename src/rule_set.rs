use cardlib::Card;
use std::cmp::Ordering;

pub trait RuleSet {
    fn compare(played: &Card, lying: &Card) -> Ordering;
    fn is_trump(card: &Card) -> bool;
    fn serves(hand: Vec<Card>, played: &Card, lying: &Card) -> bool;
}
