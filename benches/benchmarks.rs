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
    let moves: Vec<_> = game
        .get_valid_moves()
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i == &0 || i == &1 || i == &4)
        .map(|(_, m)| m)
        .collect();

    let game1 = game.clone().do_move(moves[0]);
    let game2 = game.clone().do_move(moves[1]);
    let game3 = game.clone().do_move(moves[2]);

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
        let m = game.get_valid_moves()[0];
        game = game.do_move(m);
    }

    for _ in 19..=26
    {
        let m = game.get_valid_moves()[1];
        game = game.do_move(m);
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
