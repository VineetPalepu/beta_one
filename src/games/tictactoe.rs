use std::fmt::Display;

use super::GameResult;

pub struct TicTacToe
{
    board: Vec<u32>,
    last_move: Option<TicTacToeMove>,
    rows: usize,
    cols: usize,
    num_to_win: usize,
}

impl TicTacToe
{
    pub fn new(rows: usize, cols: usize, num_to_win: usize) -> TicTacToe
    {
        TicTacToe {
            board: vec![0; rows * cols],
            last_move: None,
            rows,
            cols,
            num_to_win,
        }
    }

    fn i2p(&self, index: usize) -> Position
    {
        Position {
            row: index / self.cols,
            col: index % self.cols,
        }
    }

    fn p2i(&self, position: &Position) -> usize
    {
        position.row * self.cols + position.col
    }
}

impl Clone for TicTacToe
{
    fn clone(&self) -> Self
    {
        Self {
            board: self.board.clone(),
            last_move: self.last_move,
            rows: self.rows,
            cols: self.cols,
            num_to_win: self.num_to_win,
        }
    }
}

impl Display for TicTacToe
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        for i in 0..self.rows
        {
            for j in 0..self.cols
            {
                let index = self.p2i(&Position { row: i, col: j });
                write!(f, "{} ", self.board[index])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl super::GameState for TicTacToe
{
    type Move = TicTacToeMove;

    fn get_valid_moves(&self) -> Vec<Self::Move>
    {
        let mut moves: Vec<Self::Move> = vec![];
        for i in 0..self.board.len()
        {
            if self.board[i] == 0
            {
                moves.push(TicTacToeMove {
                    position: self.i2p(i),
                    player: self.player_to_move(),
                });
            }
        }

        moves
    }

    fn player_to_move(&self) -> u32
    {
        match &self.last_move
        {
            Some(last_move) =>
            {
                if last_move.player == 1
                {
                    2
                }
                else
                {
                    1
                }
            },
            None => 1,
        }
    }

    fn do_move(&mut self, m: Self::Move)
    {
        let index = self.p2i(&m.position);
        self.board[index] = m.player;
        self.last_move = Some(m);
    }

    fn check_win(&self) -> GameResult
    {
        if self.last_move.is_none()
        {
            return GameResult::InProgress;
        }
        let last_move = self.last_move.unwrap();

        let game_over = self.get_valid_moves().is_empty();

        let player = last_move.player;

        for dir in [(-1, -1), (-1, 0), (-1, 1), (0, 1)]
        {
            let mut consecutive = 0;
            let mut new_pos = last_move.position;

            while self.board[self.p2i(&new_pos)] == player
            {
                consecutive += 1;

                if consecutive >= self.num_to_win
                {
                    if player == 1
                    {
                        return GameResult::P1Win;
                    }
                    else
                    {
                        assert_eq!(player, 2);
                        return GameResult::P2Win;
                    }
                }

                let irow: i32 = new_pos.row.try_into().unwrap();
                let icol: i32 = new_pos.col.try_into().unwrap();

                let new_row = irow + dir.0;
                let new_col = icol + dir.1;

                if new_row < 0
                    || new_row >= self.rows.try_into().unwrap()
                    || new_col < 0
                    || new_col >= self.cols.try_into().unwrap()
                {
                    break;
                }

                new_pos.col = new_col.try_into().unwrap();
                new_pos.row = new_row.try_into().unwrap();
            }

            consecutive -= 1;
            new_pos = last_move.position;

            while self.board[self.p2i(&new_pos)] == player
            {
                consecutive += 1;
                if consecutive >= self.num_to_win
                {
                    if player == 1
                    {
                        return GameResult::P1Win;
                    }
                    else
                    {
                        assert_eq!(player, 2);
                        return GameResult::P2Win;
                    }
                }

                let irow: i32 = new_pos.row.try_into().unwrap();
                let icol: i32 = new_pos.col.try_into().unwrap();

                let new_row = irow - dir.0;
                let new_col = icol - dir.1;

                if new_row < 0
                    || new_row >= self.rows.try_into().unwrap()
                    || new_col < 0
                    || new_col >= self.cols.try_into().unwrap()
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
}

// Tic Tac Toe helper code
#[derive(Copy, Clone)]
pub struct TicTacToeMove
{
    position: Position,
    player: u32,
}

impl TicTacToeMove
{
    pub fn new(position: Position, player: u32) -> TicTacToeMove
    {
        TicTacToeMove { position, player }
    }
}

impl Display for TicTacToeMove
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Player: {}, Position: {}", self.player, self.position)
    }
}

#[derive(Copy, Clone)]
pub struct Position
{
    row: usize,
    col: usize,
}

impl Position
{
    pub fn new(row: usize, col: usize) -> Position
    {
        Position { row, col }
    }
}

impl Display for Position
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "({}, {})", self.row, self.col)
    }
}
