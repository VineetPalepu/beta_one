use core::panic;
use std::fmt::{self, Display, Formatter};

use self::board::{position::Position, Board};

use super::{GameResult, GameState};

#[derive(Clone)]
pub struct Connect4
{
    board: Board<Piece>,
    num_to_win: usize,
    open_positions: Vec<Position>,
    last_move: Option<Connect4Move>,
}

impl Connect4
{
    pub fn new(rows: usize, cols: usize, num_to_win: usize) -> Connect4
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
        // change board data based on move
        self.board[m.position] = match m.player
        {
            1 => Piece::P1,
            2 => Piece::P2,
            _ => panic!("invalid player"),
        };

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

        // if we reach the top of the column, there are no more valid
        // positions open, so remove the position from the vec
        if m.position.row == 0
        {
            self.open_positions.swap_remove(index);
        }
        // otherwise, we can continue stacking pieces on top of this one, so update the
        // position to be (row - 1, col)
        else
        {
            self.open_positions[index].row -= 1;
        }

        // update the last_move so that all other logic works
        self.last_move = Some(m);
    }

    // TODO: Refactor, lots of code reuse
    // possibly generate list of positions along each direction first then check for number
    // of consecutives found
    fn check_win(&self) -> GameResult
    {
        let last_move = match self.last_move
        {
            Some(m) => m,
            None => return GameResult::InProgress,
        };

        let player = match &last_move.player
        {
            1 => Piece::P1,
            2 => Piece::P2,
            _ => panic!("invalid player"),
        };

        let board = &self.board;

        for dir in [(-1, -1), (-1, 0), (-1, 1), (0, 1)]
        {
            let mut consecutive = 0;
            let mut new_pos = last_move.position;

            while board[new_pos] == player
            {
                consecutive += 1;

                if consecutive >= self.num_to_win
                {
                    if player == Piece::P1
                    {
                        return GameResult::P1Win;
                    }
                    else
                    {
                        debug_assert_eq!(player, Piece::P2);
                        return GameResult::P2Win;
                    }
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

            while board[new_pos] == player
            {
                consecutive += 1;
                if consecutive >= self.num_to_win
                {
                    if player == Piece::P1
                    {
                        return GameResult::P1Win;
                    }
                    else
                    {
                        debug_assert_eq!(player, Piece::P2);
                        return GameResult::P2Win;
                    }
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

impl Display for Connect4
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.board)?;

        Ok(())
    }
}
#[derive(Clone, Copy)]
pub struct Connect4Move
{
    position: Position,
    player: u32,
}

impl Display for Connect4Move
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        write!(f, "Player: {}, Position: {}", self.player, self.position)
    }
}

mod board
{
    use std::{
        fmt::{self, Display, Formatter},
        ops::{Index, IndexMut},
    };

    use self::position::Position;

    #[derive(Clone)]
    pub struct Board<T>
    {
        data: Vec<T>,
        rows: usize,
        cols: usize,
    }

    impl<T: Default> Board<T>
    {
        pub fn new(rows: usize, cols: usize) -> Board<T>
        {
            Board {
                data: (0..rows * cols).map(|_| T::default()).collect(),
                rows,
                cols,
            }
        }

        pub fn rows(&self) -> usize
        {
            self.rows
        }

        pub fn cols(&self) -> usize
        {
            self.cols
        }
    }

    impl<T: Display> Display for Board<T>
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
        {
            for row in 0..self.rows
            {
                for col in 0..self.cols
                {
                    write!(f, "{} ", self.index(Position { row, col }))?;
                }
                writeln!(f)?;
            }

            Ok(())
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

    pub mod position
    {
        use std::fmt::{self, Display, Formatter};

        #[derive(Clone, Copy)]
        pub struct Position
        {
            pub row: usize,
            pub col: usize,
        }

        impl Display for Position
        {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
            {
                write!(f, "({}, {})", self.row, self.col)
            }
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Debug)]
enum Piece
{
    #[default]
    Empty,
    P1,
    P2,
}

impl Display for Piece
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Piece::Empty => write!(f, "0"),
            Piece::P1 => write!(f, "1"),
            Piece::P2 => write!(f, "2"),
        }
    }
}
