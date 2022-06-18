use crate::games::GameState;
use std::fmt::Display;

pub mod human;

pub trait Player
{
    fn choose_move<Game>(&self, game_state: &Game) -> Game::Move
    where
        Game: GameState,
        Game::Move: Display;
}
