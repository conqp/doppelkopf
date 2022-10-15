#[derive(Debug, Eq, PartialEq)]
pub enum TurnResult {
    Ok,
    NotYourTurn,
    InvalidServe,
}
