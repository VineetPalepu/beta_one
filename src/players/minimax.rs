use std::cmp::max_by_key;

use crate::{
    games::{GameResult, GameState, Player},
    players::GamePlayer,
};

pub struct MinimaxPlayer
{
    depth: Option<usize>,
}

impl MinimaxPlayer
{
    pub fn new(depth: Option<usize>) -> MinimaxPlayer
    {
        MinimaxPlayer { depth }
    }
}

impl GamePlayer for MinimaxPlayer
{
    // TODO: choose between equivalent moves by depth?
    fn choose_move<Game>(&self, game_state: &Game) -> Game::Move
    where
        Game: crate::games::GameState,
        Game::Move: std::fmt::Display,
    {
        let mut results = vec![];
        for m in game_state.get_valid_moves()
        {
            let mut new_state = game_state.clone();
            new_state.do_move(m);
            let value = minimax(&new_state, self.depth.unwrap_or(usize::MAX));
            results.push((m, value));
        }

        results
            .iter()
            .max_by(|r1, r2| r1.1.total_cmp(&r2.1))
            .unwrap()
            .0
    }
}

fn minimax<T>(state: &T, depth: usize) -> f32
where
    T: GameState,
{
    if depth == 0
    {
        todo!("implement evauate function for game state");
        // TODO:
        // return evaluate(state);
    }
    match state.check_win()
    {
        GameResult::InProgress =>
        {},
        GameResult::Draw => return 0.0,
        GameResult::Win(_) => return f32::INFINITY,
    }

    let mut best_score = f32::NEG_INFINITY;
    for m in state.get_valid_moves()
    {
        let mut new_state = state.clone();
        new_state.do_move(m);
        let score = minimax(&new_state, depth - 1);
        best_score = f32::max(best_score, score);
    }

    -best_score
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
