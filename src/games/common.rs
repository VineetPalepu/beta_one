use super::GameState;
use board::Position;

use petgraph::dot::Dot;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

use std::collections::VecDeque;
use std::fmt::Display;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub mod board;

// TODO: accept parent pointer so graph can be added to another graph
pub fn create_game_tree<T>(state: &T, depth: Option<usize>) -> Graph<T, T::Move>
where
    T: GameState,
{
    let mut tree = Graph::new();

    let root = tree.add_node(state.clone());

    add_children(&mut tree, root, depth);

    tree
}

fn add_children<N>(tree: &mut Graph<N, N::Move>, node: NodeIndex, depth: Option<usize>)
where
    N: GameState,
{
    if depth.is_some() && depth.unwrap() == 0
    {
        return;
    }

    let state = tree
        .node_weight(node)
        .expect("couldn't get node weight")
        .clone();
    for m in state.get_valid_moves()
    {
        let mut new_state = state.clone();
        new_state.do_move(m);
        let new_node = tree.add_node(new_state);
        tree.add_edge(node, new_node, m);

        add_children(tree, new_node, depth.map(|x| x - 1));
    }
}

pub fn tree_to_file<N: Display, E: Display>(tree: Graph<N, E>, file: &str)
{
    let file = File::create(file).unwrap();
    let mut file_writer = BufWriter::new(file);
    let data = Dot::with_config(&tree, &[]);
    writeln!(file_writer, "{}", data).unwrap();
}

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
