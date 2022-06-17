use std::{
    fmt::Display,
    io::{self, Write},
    panic,
};

fn main()
{
    let mut game = TicTacToe::new(3, 3, 3);
    let p1 = HumanPlayer {};
    let p2 = HumanPlayer {};

    play(&mut game, &p1, &p2);
}

struct HumanPlayer;

fn read_number(max: usize) -> Option<usize>
{
    print!("Enter an integer in the range [0, {}): ", max);

    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input)
    {
        Ok(_) =>
        {
            let index = input.trim().parse::<usize>();
            println!("{:?}", index);
            if let Ok(index) = index
            {
                if index < max
                {
                    return Some(index);
                }
            };
        },
        Err(_) =>
        {},
    };

    None
}

impl Player for HumanPlayer
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
            println!("    {}: {}", i, m);
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

struct TicTacToe
{
    board: Vec<u32>,
    last_move: Option<TicTacToeMove>,
    rows: usize,
    cols: usize,
    num_to_win: usize,
}

impl TicTacToe
{
    fn new(rows: usize, cols: usize, num_to_win: usize) -> TicTacToe
    {
        TicTacToe {
            board: vec![0; rows * cols],
            last_move: None,
            rows,
            cols,
            num_to_win,
        }
    }

    fn i2p(&self, index: usize) -> Position
    {
        Position {
            row: index / self.cols,
            col: index % self.cols,
        }
    }

    fn p2i(&self, position: &Position) -> usize
    {
        position.row * self.cols + position.col
    }
}

// TODO: Move into same module as TicTacToe Game for encapsulation
#[derive(Copy, Clone)]
struct TicTacToeMove
{
    position: Position,
    player: u32,
}

#[derive(Copy, Clone)]
struct Position
{
    row: usize,
    col: usize,
}

impl Display for Position
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl Display for TicTacToeMove
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Player: {}, Position: {}", self.player, self.position)
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
                    position: self.i2p(i),
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
        let index = self.p2i(&m.position);
        self.board[index] = m.player;
        self.last_move = Some(m);
    }

    fn check_win(&self) -> i32
    {
        if let None = self.last_move
        {
            return -1;
        }
        let last_move = self.last_move.unwrap();

        let game_over = self.get_valid_moves().len() == 0;

        let player = last_move.player;

        for dir in [(-1, -1), (-1, 0), (-1, 1), (0, 1)]
        {
            let mut consecutive = 0;
            let mut new_pos = last_move.position;

            while self.board[self.p2i(&new_pos)] == player
            {
                consecutive += 1;

                if consecutive >= self.num_to_win
                {
                    return player.try_into().unwrap();
                }

                let irow: i32 = new_pos.row.try_into().unwrap();
                let icol: i32 = new_pos.col.try_into().unwrap();

                let new_row = irow + dir.0;
                let new_col = icol + dir.1;

                if new_row < 0
                    || new_row >= self.rows.try_into().unwrap()
                    || new_col < 0
                    || new_col >= self.cols.try_into().unwrap()
                {
                    break;
                }

                new_pos.col = new_col.try_into().unwrap();
                new_pos.row = new_row.try_into().unwrap();
            }

            consecutive -= 1;
            new_pos = last_move.position;

            while self.board[self.p2i(&new_pos)] == player
            {
                consecutive += 1;
                if consecutive >= self.num_to_win
                {
                    return player.try_into().unwrap();
                }

                let irow: i32 = new_pos.row.try_into().unwrap();
                let icol: i32 = new_pos.col.try_into().unwrap();

                let new_row = irow - dir.0;
                let new_col = icol - dir.1;

                if new_row < 0
                    || new_row >= self.rows.try_into().unwrap()
                    || new_col < 0
                    || new_col >= self.cols.try_into().unwrap()
                {
                    break;
                }

                new_pos.col = new_col.try_into().unwrap();
                new_pos.row = new_row.try_into().unwrap();
            }
        }

        if game_over
        {
            return 0;
        }

        -1
    }

    fn print_state(&self)
    {
        for i in 0..self.rows
        {
            for j in 0..self.cols
            {
                let index = self.p2i(&Position { row: i, col: j });
                print!("{} ", self.board[index]);
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
        Game: GameState,
        Game::Move: Display;
}

trait GameState
{
    type Move: Copy;

    fn get_valid_moves(&self) -> Vec<Self::Move>;

    fn get_current_player(&self) -> u32;

    fn do_move(&mut self, m: Self::Move);

    fn check_win(&self) -> i32;

    fn print_state(&self);
}

fn benchmark_players<Game>(game: &mut Game, p1: &impl Player, p2: &impl Player, iterations: u32)
where
    Game: GameState,
    Game::Move: Display,
{
    for _ in 0..iterations
    {
        play(game, p1, p2);
    }
}

fn play<Game>(game: &mut Game, p1: &impl Player, p2: &impl Player) -> i32
where
    Game: GameState,
    Game::Move: Display,
{
    while game.check_win() == -1
    {
        // Print current state
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

        // Print selected move
        println!("{}", &selected_move);

        // Update game based on the move
        game.do_move(selected_move);
    }

    // Print final game state
    game.print_state();

    // Announce winner
    let winner = game.check_win();
    if winner == 0
    {
        println!("Draw!");
    }
    else
    {
        println!("Player {} Wins!", winner);
    }

    winner
}
