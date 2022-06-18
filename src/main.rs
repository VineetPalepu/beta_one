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

    benchmark_players(&mut game, &p1, &p2, 100);
}

fn benchmark_players<Game>(game: &mut Game, p1: &impl Player, p2: &impl Player, iterations: u32)
where
    Game: GameState,
    Game::Move: Display,
{
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    let mut draws = 0;

    for _ in 0..iterations
    {
        let winner = play(game, p1, p2);
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

fn play<Game>(game: &mut Game, p1: &impl Player, p2: &impl Player) -> i32
where
    Game: GameState,
    Game::Move: Display,
{
    while game.check_win() == -1
    {
        // Print current state
        println!();
        game.print_state();

        // Let the current player pick their move
        let selected_move = match game.get_current_player()
        {
            1 => p1.choose_move(game),
            2 => p2.choose_move(game),
            n =>
            {
                panic!("invalid player: {}", n)
            },
        };

        // Print selected move
        println!("Selected Move: {}", &selected_move);

        // Update game based on the move
        game.do_move(selected_move);
    }

    // Print final game state
    game.print_state();

    // Announce winner
    let winner = game.check_win();
    if winner == 0
    {
        println!("Draw!");
    }
    else
    {
        println!("Player {} Wins!", winner);
    }

    winner
}
