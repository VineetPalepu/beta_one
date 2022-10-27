use std::fmt::{self, Display, Formatter};

use crate::players::GamePlayer;

pub mod connect4;
pub mod tictactoe;

pub mod common;

pub trait GameState: Clone + Display
{
    type Move: Copy + Display;

    fn get_valid_moves(&self) -> Vec<Self::Move>;

    fn player_to_move(&self) -> Player;

    fn do_move(self, m: Self::Move) -> Self;

    fn last_move(&self) -> Option<Self::Move>;

    fn check_win(&self) -> GameResult;

    // TODO: possibly also return GameState instead of GameResult
    fn play(
        mut self,
        p1: &mut impl GamePlayer,
        p2: &mut impl GamePlayer,
        verbose: bool,
    ) -> GameResult
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
                Player(1) => p1.choose_move(&self),
                Player(2) => p2.choose_move(&self),
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
            self = self.do_move(selected_move);
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

    fn benchmark_players(&self, p1: &mut impl GamePlayer, p2: &mut impl GamePlayer, iterations: u32)
    {
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        let mut draws = 0;

        for _ in 0..iterations
        {
            let initial_state = self.clone();

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
