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
    last_move: Option<TicTacToeMove>,
}

impl TicTacToe
{
    pub fn new(rows: usize, cols: usize, num_to_win: usize) -> TicTacToe
    {
        TicTacToe {
            board: Board::new(rows, cols),
            num_to_win,
            last_move: None,
        }
    }
}

impl GameState for TicTacToe
{
    type Move = TicTacToeMove;

    fn get_valid_moves(&self) -> Vec<Self::Move>
    {
        let mut moves = vec![];
        for row in 0..self.board.rows()
        {
            for col in 0..self.board.cols()
            {
                let pos = Position { row, col };
                if self.board[pos] == Cell::Empty
                {
                    moves.push(TicTacToeMove {
                        position: pos,
                        player: self.player_to_move(),
                    });
                }
            }
        }
        moves
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

        if self.get_valid_moves().is_empty()
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
