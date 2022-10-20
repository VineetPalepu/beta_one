use super::Player;

pub struct MCTSPlayer;

impl Player for MCTSPlayer
{
    fn choose_move<Game>(&self, game_state: &Game) -> Game::Move
    where
        Game: crate::games::GameState,
        Game::Move: std::fmt::Display 
    {
        
        
        todo!()
    }
}