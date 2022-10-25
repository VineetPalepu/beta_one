use super::Player;

pub struct MinimaxPlayer {}

impl Player for MinimaxPlayer
{
    fn choose_move<Game>(&self, game_state: &Game) -> Game::Move
    where
        Game: crate::games::GameState,
        Game::Move: std::fmt::Display,
    {
        todo!()
    }
}

// TODO: draw graph and physically work through the minimax procedure
// on paper instead of just copying from a website
fn minimax()
{
    todo!();
}
