use std::{fmt::Display, panic};

mod games;
mod players;

use games::GameState;
use players::Player;

fn main()
{
    let mut game = games::tictactoe::TicTacToe::new(3, 3, 3);
    let p1 = players::random::RandomPlayer {};
    let p2 = players::random::RandomPlayer {};

    //play(&mut game, &p1, &p2);

    benchmark_players(&mut game, &p1, &p2, 1000);
}

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
            1 => p1_wins += 1,
            2 => p2_wins += 1,
            0 => draws += 1,
            _ => println!("invalid winner: {}", winner),
        }
    }

    println!("Games: {iterations}");
    println!("P1 Wins: {p1_wins} / Draws: {draws} / P2 Wins: {p2_wins}");
}
