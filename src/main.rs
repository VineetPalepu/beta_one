use std::fmt::{format, Display};

mod games;
mod players;

use games::tictactoe::{Position, TicTacToe, TicTacToeMove};
use games::GameState;
use players::human::HumanPlayer;
use players::mcts::MCTSPlayer;
use players::random::RandomPlayer;
use players::Player;

use crate::games::GameResult;

fn main()
{
    let mut game = TicTacToe::new(3, 3, 3);
    let p1 = MCTSPlayer::new(100);
    let p2 = RandomPlayer {};
    let p3 = HumanPlayer {};

    //game.do_move(TicTacToeMove::new(Position::new(1, 1), 1));

    game.play(&p1, &p3, true);

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
