use beta_one::{
    games::{connect4::Connect4, tictactoe::TicTacToe, GameState},
    players::{minimax::MinimaxPlayer, random::RandomPlayer, GamePlayer},
};
use criterion::{criterion_group, criterion_main, Criterion};

fn minimax_tictactoe_benchmark(c: &mut Criterion)
{
    let mut group = c.benchmark_group("Tic Tac Toe");
    let mut player = MinimaxPlayer::new(None);

    let game = TicTacToe::new(3, 3, 3);
    let mut game1 = game.clone();
    game1.do_move(game1.get_valid_moves()[0]);
    let mut game2 = game.clone();
    game2.do_move(game2.get_valid_moves()[1]);
    let mut game3 = game.clone();
    game3.do_move(game3.get_valid_moves()[4]);

    group.bench_function("choose_move corner", |b| {
        b.iter(|| player.choose_move(&game1))
    });
    group.bench_function("choose_move edge", |b| {
        b.iter(|| player.choose_move(&game2))
    });
    group.bench_function("choose_move center", |b| {
        b.iter(|| player.choose_move(&game3))
    });

    group.finish();
}

fn minimax_connect4_benchmark(c: &mut Criterion)
{
    let mut player = MinimaxPlayer::new(None);
    let mut game = Connect4::new(6, 7, 4);
    for _ in 1..=18
    {
        game.do_move(game.get_valid_moves()[0]);
    }

    for _ in 19..=26
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
