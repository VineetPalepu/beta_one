use std::{
    fmt::{self, Display, Formatter},
    ops::{Index, IndexMut},
};

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
