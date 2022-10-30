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
    // let mut game = TicTacToe::new(3, 3, 3);

    // for _ in 0..5
    // {
    //     // game.do_move(game.get_valid_moves()[0]);
    //     game.do_move(*game.get_valid_moves().choose(&mut thread_rng()).unwrap());
    //     println!("{game}");
    // }

    // let minimax_player = MinimaxPlayer {};

    // let m = minimax_player.choose_move(&game);

    // println!("Selected Move: {m}");

    // let tree = create_game_tree(&game, None);
    // tree_to_file(tree, "out\\tree.dot");

    // game.do_move(m);
    // println!("{game}");

    let mut game = TicTacToe::new(3, 3, 3);
    // let mut game = TicTacToe::new(5, 5, 4);
    // let mut game = Connect4::new(6, 7, 4);

    let mcts_player = MCTSPlayer::new(15000);
    let rand_player = RandomPlayer {};
    let human_player = HumanPlayer {};
    let minimax_player = MinimaxPlayer::new(None);

    game.play(&human_player, &mcts_player, true);

    // game.benchmark_players(&minimax_player, &mcts_player, 10);
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
