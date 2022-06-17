use std::{fmt::Display, panic, io};

fn main() {
    
    let mut game = TicTacToe::new();
    let p1 = HumanPlayer{};
    let p2 = HumanPlayer{};
    
    play_game(&mut game, &p1, &p2);
}

struct HumanPlayer;

impl Player for HumanPlayer
{
    fn choose_move<Game>(&self, game_state: &Game) -> Game::Move
    where
        Game: GameState,
    {
        let moves = game_state.get_valid_moves();
        println!("{} Moves: ", moves.len());
        for m in &moves
        {
            // TODO: implement proper printing
            //println!("{}", m);
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("invalid input");

        let input: usize = input.trim().parse().expect("invalid input");

        return moves[input];
    }
}

struct TicTacToe
{
    board: Vec<u32>,
    last_move: Option<TicTacToeMove>,
    rows: usize,
    cols: usize,
}

impl TicTacToe
{
    fn new() -> TicTacToe
    {
        TicTacToe {
            board: vec![0; 9],
            last_move: None,
            rows: 3,
            cols: 3,
        }
    }
}

// TODO: Move into same module as TicTacToe Game for encapsulation
#[derive(Copy, Clone)]
struct TicTacToeMove
{
    location: usize,
    player: u32,
}

impl Display for TicTacToeMove
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Player: {}, Position: {}", self.player, self.location)
    }
}

impl GameState for TicTacToe
{
    type Move = TicTacToeMove;

    fn get_valid_moves(&self) -> Vec<Self::Move>
    {
        let mut moves: Vec<Self::Move> = vec![];
        for i in 0..self.board.len()
        {
            if self.board[i] == 0
            {
                moves.push(TicTacToeMove {
                    location: i,
                    player: self.get_current_player(),
                });
            }
        }

        moves
    }

    fn get_current_player(&self) -> u32
    {
        match &self.last_move
        {
            Some(last_move) =>
            {
                if last_move.player == 1
                {
                    2
                }
                else
                {
                    1
                }
            },
            None => 1,
        }
    }

    fn do_move(&mut self, m: Self::Move)
    {
        self.board[m.location] = m.player;
        self.last_move = Some(m);
    }

    fn print_state(&self)
    {
        for i in 0..self.rows
        {
            for j in 0..self.cols
            {
                print!("{} ", self.board[i * self.cols + j]);
            }
            println!();
        }
        println!();
    }
}

trait Player
{
    fn choose_move<Game>(&self, game_state: &Game) -> Game::Move
    where
        Game: GameState;
}

trait GameState
{
    type Move : Copy;

    fn get_valid_moves(&self) -> Vec<Self::Move>;

    fn get_current_player(&self) -> u32;

    fn do_move(&mut self, m: Self::Move);

    fn print_state(&self);
}

fn benchmark_players(game: &mut impl GameState, p1: &impl Player, p2: &impl Player, iterations: i32)
{
    for i in 0..iterations
    {
        play_game(game, p1, p2);
    }
}

fn play_game(game: &mut impl GameState, p1: &impl Player, p2: &impl Player)
{
    while !game.get_valid_moves().is_empty()
    {
        game.print_state();

        // Let the current player pick their move
        let selected_move = match game.get_current_player()
        {
            1 => p1.choose_move(game),
            2 => p2.choose_move(game),
            n =>
            {
                panic!("invalid player: {}", n)
            },
        };

        // Update game based on the move
        game.do_move(selected_move);
    }

    game.print_state();
    println!("Player {} Wins!", 1);
}
