use std::fmt::Display;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

mod games;
mod players;

use games::connect4::Connect4;
use games::tictactoe::TicTacToe;
use games::GameState;
use petgraph::data::DataMap;
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::visit::NodeRef;
use petgraph::Graph;
use players::human::HumanPlayer;
use players::mcts;
use players::mcts::MCTSPlayer;
use players::minimax;
use players::minimax::MinimaxPlayer;
use players::random::RandomPlayer;
use players::GamePlayer;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::games::common::create_game_tree;
use crate::games::common::tree_to_file;
use crate::games::GameResult;
use crate::games::Player;

// TODO: add rotation invariance for games like tic tac toe
// that is, a board should be considered the same as another if it is only a rotation or mirror image
// in 3x3 ttt this collapses the set of initial states to 3: center, corner, or edge
// TODO: use this to generate rotation invariant game tree
#[allow(unused_variables, unused_mut)]
fn main()
{
    println!("aslkdjflaskdjflskjdf");
    let board = TicTacToe::ttt_from_str("0000000000");
    println!("{board}");
}

#[allow(dead_code)]
fn println_indent<T: Display>(obj: &T, indents: usize)
{
    let indent_str = "\t".repeat(indents);
    let str = obj.to_string();

    let new_newline = format!("\n{}", indent_str);
    print!("{indent_str}");
    println!("{}", str.replace('\n', &new_newline));
}
