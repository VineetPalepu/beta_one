use crate::players::Player;

pub mod tictactoe;

pub trait GameState: Clone
{
    type Move: Copy;

    fn get_valid_moves(&self) -> Vec<Self::Move>;

    fn get_current_player(&self) -> u32;

    fn do_move(&mut self, m: Self::Move);

    fn check_win(&self) -> i32;

    fn print_state(&self);

    
    fn play(&mut self, p1: &impl Player, p2: &impl Player) -> i32
    {
        
        todo!();
    }
}
