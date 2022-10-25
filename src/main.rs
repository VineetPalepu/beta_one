use std::fmt::Display;

mod games;
mod players;

use games::connect4::Connect4;
use games::tictactoe::TicTacToe;
use games::GameState;
use players::human::HumanPlayer;
use players::mcts::MCTSPlayer;
use players::random::RandomPlayer;
use players::GamePlayer;

use crate::games::{GameResult, Player};

#[allow(unused_variables, unused_mut)]
fn main()
{
    // let mut game = TicTacToe::new(3, 3, 3);
    // let mut game = TicTacToe::new(5, 5, 4);
    let mut game = Connect4::new(6, 7, 4);

    let mcts_player = MCTSPlayer::new(300);
    let rand_player = RandomPlayer {};
    let human_player = HumanPlayer {};

    game.play(&mcts_player, &human_player, true);

    //benchmark_players(&game, &p1, &p2, 1000);
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
fn benchmark_players<Game>(game: &Game, p1: &impl GamePlayer, p2: &impl GamePlayer, iterations: u32)
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
            GameResult::Win(player) =>
            {
                if player == Player::new(1)
                {
                    p1_wins += 1;
                }
                else if player == Player::new(2)
                {
                    p2_wins += 1;
                }
            },
            GameResult::Draw => draws += 1,
            GameResult::InProgress =>
            {},
        }
    }

    println!("Games: {iterations}");
    println!("P1 Wins: {p1_wins} / Draws: {draws} / P2 Wins: {p2_wins}");
}
