use rand::Rng;

use crate::players::GamePlayer;

#[derive(Clone, Copy)]
pub struct RandomPlayer;

impl GamePlayer for RandomPlayer
{
    fn choose_move<Game>(&mut self, game_state: &Game) -> Game::Move
    where
        Game: crate::games::GameState,
        //Game::Move: std::fmt::Display,
    {
        let moves = game_state.get_valid_moves();
        let mut rng = rand::thread_rng();

        let index = rng.gen_range(0..moves.len());

        moves[index]
    }
}
