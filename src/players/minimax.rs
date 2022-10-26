use std::fmt::Debug;

use rand::{seq::SliceRandom, thread_rng};

use crate::{
    games::{GameResult, GameState, Player},
    players::GamePlayer,
};

pub struct MinimaxPlayer {}

impl GamePlayer for MinimaxPlayer
{
    fn choose_move<Game>(&self, game_state: &Game) -> Game::Move
    where
        Game: crate::games::GameState,
        Game::Move: std::fmt::Display,
    {
        minimax(game_state).0
    }
}

// TODO: add max_depth and evaluate function
// TODO: choose equivalent moves by depth
fn minimax<T>(state: &T) -> (T::Move, i128)
where
    T: GameState,
{
    match state.check_win()
    {
        GameResult::InProgress =>
        {},
        GameResult::Draw => return (state.last_move().unwrap(), 0),
        GameResult::Win(player) =>
        {
            if player == Player::new(1)
            {
                return (state.last_move().unwrap(), 1);
            }
            else
            {
                return (state.last_move().unwrap(), -1);
            }
        },
    }

    let mut results = vec![];
    for m in state.get_valid_moves()
    {
        let mut new_state = state.clone();
        new_state.do_move(m);
        results.push((m, minimax(&new_state).1));
    }

    if state.player_to_move() == Player::new(1)
    {
        // Maximizing player
        *results
            .iter()
            .max_by(|x, y| x.1.cmp(&y.1))
            .expect("state had no valid moves - should have returned at match block above")
    }
    else
    {
        // Minimizing player
        *results
            .iter()
            .min_by(|x, y| x.1.cmp(&y.1))
            .expect("state had no valid moves - should have returned at match block above")
    }
}

#[cfg(test)]
mod test
{
    use crate::games::{
        common::board::Position,
        tictactoe::{TicTacToe, TicTacToeMove},
        GameState, Player,
    };

    use super::minimax;

    fn do_move(game: &mut TicTacToe, row: usize, col: usize, player: usize)
    {
        game.do_move(TicTacToeMove {
            position: Position { row, col },
            player: Player::new(player),
        });
    }

    #[test]
    fn test_minimax()
    {
        let mut game = TicTacToe::new(3, 3, 3);
        /*
        do_move(&mut game, 0, 0, 1);
        do_move(&mut game, 2, 0, 2);
        do_move(&mut game, 0, 1, 1);
        do_move(&mut game, 2, 1, 2);
        do_move(&mut game, 1, 2, 1);
        do_move(&mut game, 1, 1, 2);
        do_move(&mut game, 2, 2, 1);
        */

        do_move(&mut game, 0, 2, 1);
        do_move(&mut game, 0, 0, 2);
        do_move(&mut game, 1, 0, 1);
        do_move(&mut game, 0, 1, 2);
        do_move(&mut game, 2, 0, 1);
        do_move(&mut game, 1, 2, 2);

        minimax(&game);
    }
}
