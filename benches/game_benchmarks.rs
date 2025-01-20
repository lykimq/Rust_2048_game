use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_2048_game::{Direction, GameState, GRID_SIZE};

fn benchmark_move_tiles(c: &mut Criterion) {
    let mut group = c.benchmark_group("move_tiles");

    // Benchmark different grid sizes
    let mut empty_state = GameState::new();
    let mut full_state = GameState::new();

    // Fill the grid for full state
    for i in 0..GRID_SIZE as usize {
        for j in 0..GRID_SIZE as usize {
            full_state.grid[i][j] = 2;
        }
    }

    group.bench_function("move_right_empty_state", |b| {
        b.iter(|| empty_state.move_tiles(black_box(Direction::Right)))
    });

    group.bench_function("move_right_full_state", |b| {
        b.iter(|| full_state.move_tiles(black_box(Direction::Right)))
    });

    group.finish();
}

fn benchmark_game_over(c: &mut Criterion) {
    let mut group = c.benchmark_group("game_over");

    let mut empty_state = GameState::new();
    let mut full_state = GameState::new();

    // Fill the grid for full state
    for i in 0..GRID_SIZE as usize {
        for j in 0..GRID_SIZE as usize {
            full_state.grid[i][j] = 2;
        }
    }

    // Fill the grid for full state with alternating numbers
    for i in 0..GRID_SIZE as usize {
        for j in 0..GRID_SIZE as usize {
            full_state.grid[i][j] = if (i + j) % 2 == 0 { 2 } else { 4 };
        }
    }

    group.bench_function("game_over_empty_state", |b| {
        b.iter(|| empty_state.check_game_over())
    });

    group.bench_function("game_over_full_state", |b| {
        b.iter(|| full_state.check_game_over())
    });

    group.finish();
}

criterion_group!(benches, benchmark_move_tiles, benchmark_game_over);
criterion_main!(benches);
