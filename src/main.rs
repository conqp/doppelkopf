mod decks;
mod game;
mod game_type;
mod player;
mod player_state;
mod reservation;
mod round;
mod round_player;
mod rule_set;
mod solo_type;
mod standard_game;
mod team;
use standard_game::TRUMPS;

fn main() {
    for card in TRUMPS {
        println!("{}", card);
    }
}
