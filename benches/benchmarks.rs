use beta_one::{
    games::{connect4::Connect4, tictactoe::TicTacToe, GameState},
    players::{minimax::MinimaxPlayer, random::RandomPlayer, GamePlayer},
};
use criterion::{criterion_group, criterion_main, Criterion};

fn minimax_benchmark(c: &mut Criterion)
{
    let mut group = c.benchmark_group("Minimax Benchmarks");
    let mut game = TicTacToe::new(3, 3, 3);
    game.do_move(game.get_valid_moves()[0]);
    let player = MinimaxPlayer::new(None);
    group.bench_function("choose_move(TicTacToe)", |b| {
        b.iter(|| {
            player.choose_move(&game);
        })
    });

    let mut game = Connect4::new(6, 7, 4);
    // TODO: seed random number so that it doesn't crash if the game ends by chance
    let random = RandomPlayer::new();
    for _ in 0..100
    {
        game.do_move(random.choose_move(&game));
    }
    group.bench_function("choose_move(Connect4)", |b| {
        b.iter(|| player.choose_move(&game))
    });
}

criterion_group!(minimax, minimax_benchmark);
criterion_main!(minimax);
