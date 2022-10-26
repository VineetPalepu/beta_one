use std::collections::VecDeque;

use board::Position;

pub mod board;

pub fn generate_line(pos: Position, dir: (i128, i128), size: (usize, usize)) -> Vec<Position>
{
    let mut positions = VecDeque::new();

    let start_pos: (i128, i128) = (pos.row.try_into().unwrap(), pos.col.try_into().unwrap());

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

pub fn on_board(pos: (i128, i128), size: (usize, usize)) -> bool
{
    pos.0 >= 0
        && pos.1 >= 0
        && pos.0 < size.0.try_into().unwrap()
        && pos.1 < size.1.try_into().unwrap()
}

pub fn tuple_to_pos(tuple: (i128, i128)) -> Position
{
    Position {
        row: tuple.0.try_into().unwrap(),
        col: tuple.1.try_into().unwrap(),
    }
}
