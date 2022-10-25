use std::fmt::{self, Display, Formatter};

use super::{
    board::{Board, Position},
    GameResult, GameState, Player,
};

#[derive(Clone)]
pub struct TicTacToe
{
    board: Board<Cell>,
    last_move: Option<TicTacToeMove>,
    num_to_win: usize,
}

impl TicTacToe
{
    pub fn new(rows: usize, cols: usize, num_to_win: usize) -> TicTacToe
    {
        TicTacToe {
            board: Board::new(rows, cols),
            last_move: None,
            num_to_win,
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

        let game_over = self.get_valid_moves().is_empty();

        let player = last_move.player;

        let board = &self.board;

        for dir in [(-1, -1), (-1, 0), (-1, 1), (0, 1)]
        {
            let mut consecutive = 0;
            let mut new_pos = last_move.position;

            while board[new_pos] == Cell::Piece(player)
            {
                consecutive += 1;

                if consecutive >= self.num_to_win
                {
                    return GameResult::Win(player);
                }

                let irow: i32 = new_pos
                    .row
                    .try_into()
                    .expect("couldn't convert index to integer");
                let icol: i32 = new_pos
                    .col
                    .try_into()
                    .expect("couldn't convert index to integer");

                let new_row = irow + dir.0;
                let new_col = icol + dir.1;

                if new_row < 0
                    || new_row >= board.rows().try_into().unwrap()
                    || new_col < 0
                    || new_col >= board.cols().try_into().unwrap()
                {
                    break;
                }

                new_pos.col = new_col.try_into().unwrap();
                new_pos.row = new_row.try_into().unwrap();
            }

            consecutive -= 1;
            new_pos = last_move.position;

            while board[new_pos] == Cell::Piece(player)
            {
                consecutive += 1;
                if consecutive >= self.num_to_win
                {
                    return GameResult::Win(player);
                }

                let irow: i32 = new_pos.row.try_into().unwrap();
                let icol: i32 = new_pos.col.try_into().unwrap();

                let new_row = irow - dir.0;
                let new_col = icol - dir.1;

                if new_row < 0
                    || new_row >= board.rows().try_into().unwrap()
                    || new_col < 0
                    || new_col >= board.cols().try_into().unwrap()
                {
                    break;
                }

                new_pos.col = new_col.try_into().unwrap();
                new_pos.row = new_row.try_into().unwrap();
            }
        }

        if game_over
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
        write!(f, "{}", self.board)?;

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

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Cell
{
    #[default]
    Empty,
    Piece(Player),
}

impl Display for Cell
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Cell::Empty => write!(f, "-"),
            Cell::Piece(p) => write!(f, "{}", p.0),
        }
    }
}
