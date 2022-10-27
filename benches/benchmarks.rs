use beta_one::{
    games::{tictactoe::TicTacToe, GameState},
    players::minimax::MinimaxPlayer,
};
use criterion::{criterion_group, criterion_main, Criterion};

fn minimax_tictactoe_benchmark(c: &mut Criterion)
{
    let mut game = TicTacToe::new(3, 3, 3);
    game.do_move(game.get_valid_moves()[0]);
    let player = MinimaxPlayer::new();
    c.bench_function("Minimax::choose_move", |b| {
        b.iter(|| {
            todo!();
        })
    });
    todo!()
}

criterion_group!(minimax, minimax_tictactoe_benchmark);
criterion_main!(minimax);
