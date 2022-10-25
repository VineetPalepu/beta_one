use std::{
    collections::VecDeque,
    fmt::{self, Display, Formatter},
};

use super::{
    board::{Board, Cell, Position},
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

    fn do_move(&mut self, m: Self::Move)
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

        fn generate_line(pos: Position, dir: (i128, i128), size: (usize, usize)) -> Vec<Position>
        {
            let mut positions = VecDeque::new();

            let start_pos: (i128, i128) =
                (pos.row.try_into().unwrap(), pos.col.try_into().unwrap());

            fn on_board(pos: (i128, i128), size: (usize, usize)) -> bool
            {
                return pos.0 >= 0
                    && pos.1 >= 0
                    && pos.0 < size.0.try_into().unwrap()
                    && pos.1 < size.1.try_into().unwrap();
            }

            fn tuple_to_pos(tuple: (i128, i128)) -> Position
            {
                Position {
                    row: tuple.0.try_into().unwrap(),
                    col: tuple.1.try_into().unwrap(),
                }
            }

            let mut pos = start_pos;
            while on_board(pos, size)
            {
                positions.push_front(tuple_to_pos(pos));
                pos.0 += dir.0;
                pos.1 += dir.1;
            }

            // start_pos is the last element, which gets added again in the next loop so remove to prevent duplicate
            positions.pop_back();

            pos = start_pos;
            while on_board(pos, size)
            {
                positions.push_back(tuple_to_pos(pos));
                pos.0 -= dir.0;
                pos.1 -= dir.1;
            }

            positions.into_iter().collect()
        }

        for dir in [(-1, -1), (-1, 0), (-1, 1), (0, 1)]
        {
            let line = generate_line(start_pos, dir, (board.rows(), board.cols()));
            let mut consecutive = 0;
            for pos in line
            {
                if board[pos] == Cell::Piece(player)
                {
                    consecutive += 1;
                }
                else
                {
                    consecutive = 0;
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
        write!(f, "Result: {}", self.check_win())?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct TicTacToeMove
{
    position: Position,
    player: Player,
}

impl Display for TicTacToeMove
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}, Position: {}", self.player, self.position)
    }
}
