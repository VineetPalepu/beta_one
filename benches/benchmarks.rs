use beta_one::{
    games::{connect4::Connect4, tictactoe::TicTacToe, GameState},
    players::{minimax::MinimaxPlayer, random::RandomPlayer, GamePlayer},
};
use criterion::{criterion_group, criterion_main, Criterion};

fn minimax_tictactoe_benchmark(c: &mut Criterion)
{
    let mut player = MinimaxPlayer::new(None);
    let mut game = TicTacToe::new(3, 3, 3);
    game.do_move(game.get_valid_moves()[0]);
    c.bench_function("choose_move(TicTacToe)", |b| {
        b.iter(|| {
            player.choose_move(&game);
        })
    });
}

fn minimax_connect4_benchmark(c: &mut Criterion)
{
    let mut player = MinimaxPlayer::new(None);
    let mut game = Connect4::new(6, 7, 4);
    for i in 1..=18
    {
        game.do_move(game.get_valid_moves()[0]);
    }

    for i in 19..=26
    {
        game.do_move(game.get_valid_moves()[1]);
    }

    c.bench_function("choose_move(Connect4)", |b| {
        b.iter(|| player.choose_move(&game))
    });
}

criterion_group!(
    minimax,
    minimax_tictactoe_benchmark,
    minimax_connect4_benchmark
);
criterion_main!(minimax);
