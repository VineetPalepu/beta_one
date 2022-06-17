use std::panic;

use rand::Rng;

fn main()
{
    let mut board = vec![0; 9];
    let mut open_positions: Vec<usize> = (0..9).collect();

    let mut current_player = 1;

    while open_positions.len() > 0
    {
        print_board(&board);
        println!("Remaining Moves: {:?}", open_positions);
        let mut rng = rand::thread_rng();
        let move_index = rng.gen_range(0..open_positions.len());
        board[open_positions[move_index]] = current_player;

        open_positions.remove(move_index);

        current_player = if current_player == 1 { 2 } else { 1 };
    }

    print_board(&board);
}

fn print_board(board: &Vec<i32>)
{
    for row in 0..3
    {
        let row_start = row * 3;
        let row_end = row_start + 3;
        println!("{:?}", &board[row_start..row_end]);
    }
    println!();
}

trait Player
{
    fn choose_move<T: GameState>(&self, game_state: &T) -> T::Move;
}

trait GameState
{
    type Move;

    fn get_valid_moves(&self) -> Vec<Self::Move>;
    
    fn get_current_player(&self) -> i32;

    fn do_move(&self, m: Self::Move);
}


fn benchmark_players(game: &impl GameState, p1: &impl Player, p2: &impl Player, iterations: i32)
{
    for i in 0..iterations
    {
        play_single_game(game, p1, p2);
    }
}

fn play_single_game(game: &impl GameState, p1: &impl Player, p2: &impl Player)
{

    while !game.get_valid_moves().is_empty()
    {
        // Print Current State

        // Let the current player pick their move
        let selected_move = match game.get_current_player()
        {
            1 => {
                p1.choose_move(game)
            },
            2 => {
                p2.choose_move(game)
            },
            n => { panic!("invalid player: {}", n)},
        };

        // Update game based on the move
        game.do_move(selected_move);
    }
    // Print State
}