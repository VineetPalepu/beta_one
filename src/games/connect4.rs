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

#[derive(Clone, Copy)]
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
    num_to_win: usize,
    open_positions: Vec<Position>,
    last_move: Option<Connect4Move>,
}

impl Connect4
{
    fn new(rows: usize, cols: usize, num_to_win: usize) -> Connect4
    {
        let board = Board::new(rows, cols);
        let open_positions = (0..cols)
            .map(|i| Position { row: rows - 1, col: i })
            .collect();

        Connect4 {
            board,
            num_to_win,
            open_positions,
            last_move: None,
        }
    }
}

#[derive(Clone, Copy)]
struct Connect4Move
{
    position: Position,
    player: u32,
}

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
        self.open_positions
            .iter()
            .map(|p| Connect4Move {
                position: *p,
                player: self.player_to_move(),
            })
            .collect()
    }

    fn player_to_move(&self) -> u32
    {
        match self.last_move
        {
            #[rustfmt::skip]
            Some(m) => if m.player == 2 {1} else {2},
            None => 1,
        }
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
