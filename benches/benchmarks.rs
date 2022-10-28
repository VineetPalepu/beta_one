use beta_one::{
    games::{connect4::Connect4, tictactoe::TicTacToe, GameState},
    players::{mcts::MCTSPlayer, minimax::MinimaxPlayer, random::RandomPlayer, GamePlayer},
};
use criterion::{criterion_group, criterion_main, Criterion};

fn minimax_tictactoe_benchmark(c: &mut Criterion)
{
    let mut group = c.benchmark_group("Minimax/Tic Tac Toe");
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

    group.bench_function("choose_move(corner)", |b| {
        b.iter(|| player.choose_move(&game1))
    });
    group.bench_function("choose_move(edge)", |b| {
        b.iter(|| player.choose_move(&game2))
    });
    group.bench_function("choose_move(center)", |b| {
        b.iter(|| player.choose_move(&game3))
    });

    group.finish();
}

fn minimax_connect4_benchmark(c: &mut Criterion)
{
    let mut group = c.benchmark_group("Minimax/Connect 4");

    let mut player = MinimaxPlayer::new(None);
    let mut game = Connect4::new(6, 7, 4);
    for i in 1..=26
    {
        let m = game.get_valid_moves()[if i <= 18 { 0 } else { 1 }];
        game = game.do_move(m);
    }

    group.bench_function("choose_move", |b| b.iter(|| player.choose_move(&game)));

    group.finish();
}

fn mcts_tictactoe_benchmark(c: &mut Criterion)
{
    let mut group = c.benchmark_group("MCTS/Tic Tac Toe");

    let mut player = MCTSPlayer::new(300).set_player(RandomPlayer::from_seed(234));
    let game = TicTacToe::new(3, 3, 3);

    group.bench_function("choose_move", |b| b.iter(|| player.choose_move(&game)));

    group.finish();
}

fn mcts_connect4_benchmark(c: &mut Criterion)
{
    let mut group = c.benchmark_group("MCTS/Connect 4");

    let mut player = MCTSPlayer::new(300).set_player(RandomPlayer::from_seed(234));
    let game = Connect4::new(6, 7, 4);

    group.bench_function("choose_move", |b| b.iter(|| player.choose_move(&game)));

    group.finish();
}

fn play_tictactoe_mcts(c: &mut Criterion)
{
    let mut group = c.benchmark_group("MCTS/Tic Tac Toe");

    let game = TicTacToe::new(3, 3, 3);

    let player = MCTSPlayer::new(100).set_player(RandomPlayer::from_seed(234));
    group.bench_function("play(100 iters)", |b| {
        b.iter(|| {
            game.clone()
                .play(&mut player.clone(), &mut player.clone(), false)
        })
    });

    let player = MCTSPlayer::new(300).set_player(RandomPlayer::from_seed(234));
    group.bench_function("play(300 iters)", |b| {
        b.iter(|| {
            game.clone()
                .play(&mut player.clone(), &mut player.clone(), false)
        })
    });

    group.finish();
}

fn play_connect4_mcts(c: &mut Criterion)
{
    let mut group = c.benchmark_group("MCTS/Connect 4");

    let game = Connect4::new(6, 7, 4);

    let player = MCTSPlayer::new(100).set_player(RandomPlayer::from_seed(234));
    group.bench_function("play(100 iters)", |b| {
        b.iter(|| {
            game.clone()
                .play(&mut player.clone(), &mut player.clone(), false)
        })
    });
    let player = MCTSPlayer::new(300).set_player(RandomPlayer::from_seed(234));
    group.bench_function("play(300 iters)", |b| {
        b.iter(|| {
            game.clone()
                .play(&mut player.clone(), &mut player.clone(), false)
        })
    });

    group.finish();
}
criterion_group!(
    minimax,
    minimax_tictactoe_benchmark,
    minimax_connect4_benchmark
);
criterion_group!(mcts, mcts_tictactoe_benchmark, mcts_connect4_benchmark);

criterion_group!(games, play_tictactoe_mcts, play_connect4_mcts);

criterion_main!(minimax, mcts, games);
