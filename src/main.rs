use std::fmt::{Display, format};

mod games;
mod players;

use games::GameState;
use games::tictactoe::{TicTacToe, Position, TicTacToeMove};
use players::Player;
use players::human::HumanPlayer;
use players::random::RandomPlayer;

fn main()
{
    let mut game = TicTacToe::new(3, 3, 3);
    let p1 = RandomPlayer {};
    let p2 = RandomPlayer {};

    game.do_move(TicTacToeMove::new(Position::new(1, 1), 1));

    //play(&mut game, &p1, &p2);

    benchmark_players(&game, &p1, &p2, 1000);
}

fn println_indent<T: Display>(obj: &T, indents: usize)
{
    let indent_str = "\t".repeat(indents);
    let str = obj.to_string();

    let new_newline = format!("\n{}", indent_str);
    print!("{indent_str}");
    println!("{}", str.replace("\n", &new_newline));
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
