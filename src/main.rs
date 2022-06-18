use std::{fmt::Display, panic};

mod games;
mod players;

use games::GameState;
use players::Player;

fn main()
{
    let mut game = games::tictactoe::TicTacToe::new(3, 3, 3);
    let p1 = players::human::HumanPlayer {};
    let p2 = players::human::HumanPlayer {};

    play(&mut game, &p1, &p2);
}

fn benchmark_players<Game>(game: &mut Game, p1: &impl Player, p2: &impl Player, iterations: u32)
where
    Game: GameState,
    Game::Move: Display,
{
    for _ in 0..iterations
    {
        play(game, p1, p2);
    }
}

fn play<Game>(game: &mut Game, p1: &impl Player, p2: &impl Player) -> i32
where
    Game: GameState,
    Game::Move: Display,
{
    while game.check_win() == -1
    {
        // Print current state
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
        println!("{}", &selected_move);

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
