use rand::Rng;

use crate::players::GamePlayer;

pub struct RandomPlayer;

impl GamePlayer for RandomPlayer
{
    fn choose_move<Game>(&self, game_state: &Game) -> Game::Move
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
