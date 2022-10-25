use std::fmt::{self, Display, Formatter};

use crate::players::GamePlayer;

pub mod connect4;
pub mod tictactoe;

pub mod board;

pub trait GameState: Clone + Display
{
    type Move: Copy + Display;

    fn get_valid_moves(&self) -> Vec<Self::Move>;

    fn player_to_move(&self) -> Player;

    fn do_move(&mut self, m: Self::Move);

    fn last_move(&self) -> Option<Self::Move>;

    fn check_win(&self) -> GameResult;

    fn play(&mut self, p1: &impl GamePlayer, p2: &impl GamePlayer, verbose: bool) -> GameResult
    {
        while self.check_win() == GameResult::InProgress
        {
            // Print current state
            if verbose
            {
                println!("{}", self);
            }

            // Let the current player pick their move
            let selected_move = match self.player_to_move()
            {
                Player(1) => p1.choose_move(self),
                Player(2) => p2.choose_move(self),
                Player(n) =>
                {
                    panic!("invalid player: {}", n)
                },
            };

            // Print selected move
            if verbose
            {
                println!("Selected Move: {}", &selected_move);
            }

            // Update game based on the move
            self.do_move(selected_move);
        }

        // Print final game state
        if verbose
        {
            println!("{}", self);
        }

        // Announce winner
        let game_result = self.check_win();
        if verbose
        {
            println!("{game_result}");
        }

        game_result
    }
}

#[derive(PartialEq, Eq)]
pub enum GameResult
{
    InProgress,
    Draw,
    Win(Player),
}

impl Display for GameResult
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        write!(
            f,
            "{}",
            match self
            {
                GameResult::InProgress => String::from("Game in Progress"),
                GameResult::Draw => String::from("Draw"),
                GameResult::Win(player) => format!("{player} Wins"),
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Player(usize);

impl Player
{
    pub fn new(id: usize) -> Player
    {
        Player(id)
    }
}

impl Display for Player
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        write!(f, "Player {}", self.0)
    }
}
