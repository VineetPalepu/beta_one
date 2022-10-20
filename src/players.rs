use std::fmt::Display;

use crate::games::GameState;

pub mod human;
pub mod random;
pub mod mcts;

pub trait Player
{
    fn choose_move<Game>(&self, game_state: &Game) -> Game::Move
    where
        Game: GameState,
        Game::Move: Display;
}
