use cardlib::Card;
use std::cmp::Ordering;

pub trait RuleSet {
    fn compare(&self, played: &Card, lying: &Card) -> Ordering;
    fn serves(&self, hand: &[Card], played: &Card, lying: &Card) -> bool;
}
