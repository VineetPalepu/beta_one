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
        minimax(game_state, 0).0
    }
}

// TODO: add max_depth and evaluate function
// TODO: choose between equivalent moves by depth?
fn minimax<T>(state: &T, depth: usize) -> (T::Move, f32)
where
    T: GameState,
{
    match state.check_win()
    {
        GameResult::InProgress =>
        {},
        GameResult::Draw => return (state.last_move().unwrap(), 0.0),
        GameResult::Win(player) => return (state.last_move().unwrap(), f32::INFINITY),
    }

    let mut results = vec![];
    for m in state.get_valid_moves()
    {
        let mut new_state = state.clone();
        new_state.do_move(m);
        // multiply minimax value by -1 since we don't track separate min and max players
        results.push((m, -1.0 * minimax(&new_state, depth + 1).1));
    }

    // TEMP: DEBUG
    if depth == 0
    {
        print!("Results: ");
        for r in results.iter()
        {
            print!("({}, {}) ", r.0, r.1);
        }
        println!();
    }

    // Maximizing player
    *results
        .iter()
        // sort by minimum: a winning child node evaluaes to +inf,
        // and when added to results the value is multiplied by -1
        // thus the best move will have the smallest value
        .min_by(|x, y| {
            x.1.partial_cmp(&y.1)
                .unwrap_or_else(|| panic!("couldn't compare {} and {}", x.1, y.1))
        })
        .expect("state had no valid moves - should have returned at match block above")
}

#[cfg(test)]
mod test
{
    use crate::games::{
        common::{board::Position, create_game_tree, tree_to_file},
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
        do_move(&mut game, 0, 0, 1);
        do_move(&mut game, 2, 0, 2);
        do_move(&mut game, 0, 1, 1);
        do_move(&mut game, 2, 1, 2);
        do_move(&mut game, 1, 2, 1);
        do_move(&mut game, 1, 1, 2);
        do_move(&mut game, 2, 2, 1);

        /*
        do_move(&mut game, 0, 2, 1);
        do_move(&mut game, 0, 0, 2);
        do_move(&mut game, 1, 0, 1);
        do_move(&mut game, 0, 1, 2);
        do_move(&mut game, 2, 0, 1);
        do_move(&mut game, 1, 2, 2);
        */

        minimax(&game, 0);
        tree_to_file(create_game_tree(&game, None), "out\\tree.dot")
    }
}
