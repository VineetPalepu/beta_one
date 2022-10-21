use std::fmt::{Display, Formatter, self};

use super::GameState;



#[derive(Clone)]
struct Connect4
{

}

#[derive(Clone, Copy)]
struct Connect4Move
{

}

impl Display for Connect4Move
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl GameState for Connect4
{
    type Move = Connect4Move;

    fn get_valid_moves(&self) -> Vec<Self::Move> {
        todo!()
    }

    fn player_to_move(&self) -> u32 {
        todo!()
    }

    fn do_move(&mut self, m: Self::Move) {
        todo!()
    }

    fn check_win(&self) -> super::GameResult {
        todo!()
    }
}

impl Display for Connect4
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}