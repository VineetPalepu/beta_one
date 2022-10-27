use rand::{
    rngs::{StdRng, ThreadRng},
    thread_rng, Rng, SeedableRng,
};

use crate::{games::GameState, players::GamePlayer};

#[derive(Clone)]
pub struct RandomPlayer
{
    rng: StdRng,
}

impl RandomPlayer
{
    pub fn new() -> RandomPlayer
    {
        RandomPlayer::from_seed(thread_rng().gen())
    }

    pub fn from_seed(seed: u64) -> RandomPlayer
    {
        RandomPlayer {
            rng: StdRng::seed_from_u64(seed),
        }
    }
}

impl GamePlayer for RandomPlayer
{
    fn choose_move<T>(&mut self, game_state: &T) -> T::Move
    where
        T: crate::games::GameState,
    {
        let moves = game_state.get_valid_moves();

        let index = self.rng.gen_range(0..moves.len());

        moves[index]
    }
}
