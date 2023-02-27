use std::{
    collections::VecDeque,
    fmt::{self, Display, Formatter},
};

use crate::games::{
    common::{
        board::{Board, Cell, Position},
        generate_line,
    },
    GameResult, GameState, Player,
};

#[derive(Clone)]
pub struct TicTacToe
{
    board: Board<Cell>,
    num_to_win: usize,
    open_positions: Vec<Position>,
    last_move: Option<TicTacToeMove>,
}

impl TicTacToe
{
    pub fn new(rows: usize, cols: usize, num_to_win: usize) -> TicTacToe
    {
        let mut open_positions = vec![];
        for row in 0..rows
        {
            for col in 0..cols
            {
                open_positions.push(Position { row, col });
            }
        }

        TicTacToe {
            board: Board::new(rows, cols),
            num_to_win,
            open_positions,
            last_move: None,
        }
    }
}

impl GameState for TicTacToe
{
    type Move = TicTacToeMove;

    fn get_valid_moves(&self) -> Vec<Self::Move>
    {
        // if game is over return empty vec because there are no valid moves
        if self.check_win() != GameResult::InProgress
        {
            return vec![];
        }

        self.open_positions
            .iter()
            .map(|p| TicTacToeMove {
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
                if last_move.player.id == 1
                {
                    Player { id: 2 }
                }
                else
                {
                    Player { id: 1 }
                }
            },
            None => Player { id: 1 },
        }
    }

    fn do_move(mut self, m: Self::Move) -> Self
    {
        self.board[m.position] = Cell::Piece(m.player);
        self.last_move = Some(m);

        let index = self
            .open_positions
            .iter()
            .position(|&p| p == m.position)
            .expect("couldn't find move");

        // can change to swap_remove for better performance if necessary
        self.open_positions.remove(index);

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
            let line = generate_line(start_pos, dir, (board.rows(), board.cols()));
            let mut consecutive = 0;
            for pos in line
            {
                match board[pos] == Cell::Piece(player)
                {
                    true => consecutive += 1,
                    false => consecutive = 0,
                }

                if consecutive >= self.num_to_win
                {
                    return GameResult::Win(player);
                }
            }
        }

        // if no one has won yet, and there is no place left to play, it's a draw
        if self.open_positions.is_empty()
        {
            return GameResult::Draw;
        }

        // if it's not a draw, then the game is still in progress
        GameResult::InProgress
    }

    fn last_move(&self) -> Option<Self::Move>
    {
        self.last_move
    }
}

impl Display for TicTacToe
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
pub struct TicTacToeMove
{
    // TODO: create better public interface or determine if this needs to be public
    // while still allowing tests to be done in different modules
    pub(crate) position: Position,
    pub(crate) player: Player,
}

impl Display for TicTacToeMove
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}, Position: {}", self.player, self.position)
    }
}
