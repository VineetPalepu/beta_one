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
use players::mcts::MCTSPlayer;
use players::random::RandomPlayer;
use players::Player;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::games::GameResult;

// TODO: accept parent pointer so graph can be added to another graph
fn create_game_tree<T>(state: &T, depth: Option<usize>) -> Graph<T, T::Move>
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

        add_children(
            tree,
            new_node,
            match depth
            {
                Some(depth) => Some(depth - 1),
                None => None,
            },
        );
    }
}

#[allow(unused_variables, unused_mut)]
fn main()
{
    let mut game = TicTacToe::new(3, 3, 3);
    game.do_move(game.get_valid_moves()[0]);
    game.do_move(game.get_valid_moves()[0]);
    game.do_move(game.get_valid_moves()[0]);
    game.do_move(game.get_valid_moves()[0]);

    // TODO: make game tree explorer using graph viz that shows one node (and its neighbors) at a time and lets you navigate to
    // parent or children notes
    // should be live generated so that it only generates to the depth needed

    // TODO: update GameState Display trait to show other information like GameResult and current player
    // TODO: serialize and deserialize for GameState so you can save and explore a game tree later

    let tree = create_game_tree(&game, None);
    tree_to_file(tree, "out\\tree.dot")

    /*
    let mut game = TicTacToe::new(5, 5, 4);
    let mut game = Connect4::new(6, 7, 4);

    let mcts_player = MCTSPlayer::new(3000);
    let rand_player = RandomPlayer {};
    let human_player = HumanPlayer {};

    game.play(&mcts_player, &human_player, true);
    */

    //benchmark_players(&game, &p1, &p2, 1000);
}

fn tree_to_file<N: Display, E: Display>(tree: Graph<N, E>, file: &str)
{
    let file = File::create(file).unwrap();
    let mut file_writer = BufWriter::new(file);
    let data = Dot::new(&tree);
    writeln!(file_writer, "{}", data).unwrap();
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

#[allow(dead_code)]
fn benchmark_players<Game>(game: &Game, p1: &impl Player, p2: &impl Player, iterations: u32)
where
    Game: GameState,
    Game::Move: Display,
{
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    let mut draws = 0;

    for _ in 0..iterations
    {
        let mut initial_state = game.clone();

        let winner = initial_state.play(p1, p2, false);
        match winner
        {
            GameResult::P1Win => p1_wins += 1,
            GameResult::P2Win => p2_wins += 1,
            GameResult::Draw => draws += 1,
            GameResult::InProgress =>
            {},
        }
    }

    println!("Games: {iterations}");
    println!("P1 Wins: {p1_wins} / Draws: {draws} / P2 Wins: {p2_wins}");
}
