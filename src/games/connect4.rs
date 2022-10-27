use core::panic;
use std::fmt::{self, Display, Formatter};

use crate::games::{
    common::board::{Board, Cell, Position},
    GameResult, GameState, Player,
};

use super::common::generate_line;

#[derive(Clone)]
pub struct Connect4
{
    board: Board<Cell>,
    num_to_win: usize,
    open_positions: Vec<Position>,
    last_move: Option<Connect4Move>,
}

impl Connect4
{
    pub fn new(rows: usize, cols: usize, num_to_win: usize) -> Connect4
    {
        let open_positions = (0..cols)
            .map(|i| Position { row: rows - 1, col: i })
            .collect();

        Connect4 {
            board: Board::new(rows, cols),
            num_to_win,
            open_positions,
            last_move: None,
        }
    }
}

impl GameState for Connect4
{
    type Move = Connect4Move;

    fn get_valid_moves(&self) -> Vec<Self::Move>
    {
        // if game is over no moves allowed
        if self.check_win() != GameResult::InProgress
        {
            return vec![];
        }
        self.open_positions
            .iter()
            .map(|p| Connect4Move {
                position: *p,
                player: self.player_to_move(),
            })
            .collect()
    }

    fn player_to_move(&self) -> Player
    {
        match self.last_move
        {
            Some(last_move) =>
            {
                if last_move.player == Player(1)
                {
                    Player(2)
                }
                else
                {
                    Player(1)
                }
            },
            None => Player(1),
        }
    }

    fn do_move(mut self, m: Self::Move) -> Self
    {
        // change board data based on move
        self.board[m.position] = Cell::Piece(m.player);

        // update the last_move so that all other logic works
        self.last_move = Some(m);

        // find the index of the position played so we can modify / delete it
        let index = self
            .open_positions
            .iter()
            // there is only one position open in each column so only need to check that the column matches
            .position(|pos| pos.col == m.position.col);

        let index = match index
        {
            Some(i) => i,
            None => panic!("the move: {m} was not a valid move"),
        };

        if m.position.row == 0
        {
            // if we reach the top of the column, there are no more valid
            // positions open, so remove the position from the vec
            //
            // can switch to swap_remove for performance if necessary
            self.open_positions.remove(index);
        }
        else
        {
            // otherwise, we can continue stacking pieces on top of this one, so update the
            // position to be (row - 1, col)
            self.open_positions[index].row -= 1;
        }

        self
    }

    fn check_win(&self) -> GameResult
    {
        let last_move = match self.last_move
        {
            Some(m) => m,
            None => return GameResult::InProgress,
        };

        let board = &self.board;

        let player = last_move.player;
        let start_pos = last_move.position;

        for dir in [(-1, -1), (-1, 0), (-1, 1), (0, 1)]
        {
            // get a list of all the cells on the row, column, or diagonal going through start_pos
            let line = generate_line(start_pos, dir, (board.rows(), board.cols()));
            let mut consecutive = 0;
            for pos in line
            {
                if board[pos] == Cell::Piece(player)
                {
                    consecutive += 1;

                    if consecutive >= self.num_to_win
                    {
                        return GameResult::Win(player);
                    }
                }
                else
                {
                    consecutive = 0;
                }
            }
        }

        if self.open_positions.is_empty()
        {
            return GameResult::Draw;
        }

        GameResult::InProgress
    }

    fn last_move(&self) -> Option<Self::Move>
    {
        self.last_move
    }
}

impl Display for Connect4
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        writeln!(f, "Board: ")?;
        write!(f, "{}", self.board)?;
        if self.check_win() == GameResult::InProgress
        {
            writeln!(f, "Next Player: {}", self.player_to_move())?;
        }
        else
        {
            writeln!(f, "Result: {}", self.check_win())?;
        }
        Ok(())
    }
}
#[derive(Clone, Copy)]
pub struct Connect4Move
{
    position: Position,
    player: Player,
}

impl Display for Connect4Move
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}, Position: {}", self.player, self.position)
    }
}
