use std::{
    fmt::{self, Display, Formatter},
    ops::{Index, IndexMut},
};

use super::GameState;

#[derive(Clone)]
struct Board<T>
{
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

struct Position
{
    row: usize,
    col: usize,
}

impl<T: Default> Board<T>
{
    fn new(rows: usize, cols: usize) -> Board<T>
    {
        Board {
            data: (0..rows * cols).map(|_| T::default()).collect(),
            rows,
            cols,
        }
    }
}

impl<T> Index<Position> for Board<T>
{
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output
    {
        &self.data[index.row * self.cols + index.col]
    }
}

impl<T> IndexMut<Position> for Board<T>
{
    fn index_mut(&mut self, index: Position) -> &mut Self::Output
    {
        &mut self.data[index.row * self.cols + index.col]
    }
}

#[derive(Clone, Copy, Default)]
enum Piece
{
    #[default]
    Empty,
    P1,
    P2,
}

#[derive(Clone)]
struct Connect4
{
    board: Board<Piece>,
}

#[derive(Clone, Copy)]
struct Connect4Move {}

impl Display for Connect4Move
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        todo!()
    }
}

impl GameState for Connect4
{
    type Move = Connect4Move;

    fn get_valid_moves(&self) -> Vec<Self::Move>
    {
        todo!()
    }

    fn player_to_move(&self) -> u32
    {
        todo!()
    }

    fn do_move(&mut self, m: Self::Move)
    {
        todo!()
    }

    fn check_win(&self) -> super::GameResult
    {
        todo!()
    }
}

impl Display for Connect4
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        todo!()
    }
}
