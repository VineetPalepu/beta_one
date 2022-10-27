use std::fmt::Display;

use crate::games::GameState;

pub mod human;
pub mod mcts;
pub mod minimax;
pub mod random;

pub trait GamePlayer: Clone
{
    fn choose_move<Game>(&mut self, game_state: &Game) -> Game::Move
    where
        Game: GameState,
        Game::Move: Display;
}
