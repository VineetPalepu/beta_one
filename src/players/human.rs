use std::fmt::Display;
use std::io::{self, Write};

use super::GamePlayer;
use crate::games::GameState;

pub struct HumanPlayer;

impl GamePlayer for HumanPlayer
{
    fn choose_move<Game>(&self, game_state: &Game) -> Game::Move
    where
        Game: GameState,
        Game::Move: Display,
    {
        let moves = game_state.get_valid_moves();
        println!("{} Moves: ", moves.len());
        for (i, m) in moves.iter().enumerate()
        {
            println!("    Move {i}: {m}");
        }

        loop
        {
            if let Some(index) = read_number(moves.len())
            {
                return moves[index];
            }
        }
    }
}

fn read_number(max: usize) -> Option<usize>
{
    print!("Enter an integer in the range [0, {}): ", max);

    io::stdout().flush().unwrap();

    let mut input = String::new();

    if io::stdin().read_line(&mut input).is_ok()
    {
        let index = input.trim().parse::<usize>();
        //println!("{:?}", index);

        if let Ok(index) = index
        {
            if index < max
            {
                return Some(index);
            }
        };
    }

    None
}
