use std::fmt::Display;

use crate::players::Player;

pub mod tictactoe;

pub trait GameState: Clone + Display
{
    type Move: Copy + Display;

    fn get_valid_moves(&self) -> Vec<Self::Move>;

    fn get_current_player(&self) -> u32;

    fn do_move(&mut self, m: Self::Move);

    fn check_win(&self) -> i32;

    fn play(&mut self, p1: &impl Player, p2: &impl Player, verbose: bool) -> i32
    {
        while self.check_win() == -1
        {
            // Print current state
            if verbose
            {
                println!("{}", self);
            }

            // Let the current player pick their move
            let selected_move = match self.get_current_player()
            {
                1 => p1.choose_move(self),
                2 => p2.choose_move(self),
                n =>
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
        let winner = self.check_win();
        if verbose
        {
            if winner == 0
            {
                println!("Draw!");
            }
            else
            {
                println!("Player {} Wins!", winner);
            }
        }

        winner
    }
}
