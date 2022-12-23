use std::env;
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
    let board_string = env::args()
        .nth(1)
        .expect("need string to construct tic tac toe board");
    /*
    let str1 = "0000000000";
    let str2 = "1000000001";
    let str3 = "1000200005";
    let str4 = "1100200002";
    let strs = vec![str1, str2, str3, str4];

    for string in strs
    {
        let board = TicTacToe::ttt_from_str(string);
        println!("{board:#?}");
        println!("--------------------------------")
    }
    */
    let mut game_state = TicTacToe::ttt_from_str(&board_string);
    //println!("received state:\n{game_state}");
    match game_state.check_win()
    {
        GameResult::InProgress =>
        {
            let player = MinimaxPlayer::new(None);
            let chosen_move = player.choose_move::<TicTacToe>(&game_state);
            game_state.do_move(chosen_move);

            //println!("new state:\n{game_state}");
            println!("{}", game_state.ttt_to_str());
        },
        GameResult::Draw => println!("DRAW"),
        GameResult::Win(player) => println!("{player}"),
    }
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
