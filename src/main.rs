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
