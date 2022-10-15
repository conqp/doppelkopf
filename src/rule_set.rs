use cardlib::Card;
use std::cmp::Ordering;

pub trait RuleSet {
    fn compare(&self, played: &Card, lying: &Card) -> Ordering;
    fn is_trump(&self, card: &Card) -> bool;
    fn serves(&self, hand: &Vec<Card>, played: &Card, lying: &Card) -> bool;
}
