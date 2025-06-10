// ============================================================================
// 2048 Game Performance Benchmarks
// ============================================================================
//
// This file contains microbenchmarks for the core game operations of the 2048 game.
// The benchmarks are designed to measure the performance of the most computationally
// intensive operations that occur during gameplay.
//
// BENCHMARKING APPROACH:
// ----------------------
// We use Criterion.rs, a statistics-driven benchmarking framework that provides:
// - Robust statistical analysis with outlier detection
// - Automatic sample size determination
// - HTML reports with detailed performance metrics
// - Protection against compiler optimizations via black_box()
//
// The benchmarks focus on two critical game operations:
// 1. Tile movement logic (move_tiles method)
// 2. Game over detection (check_game_over method)
//
// These operations are benchmarked under different grid states to understand
// performance characteristics across various gameplay scenarios.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_2048_game::{Direction, GameState, GRID_SIZE};

/// Benchmarks the tile movement algorithm under different grid conditions
///
/// WHAT IS BEING BENCHMARKED:
/// The move_tiles() method, which is the core gameplay mechanic responsible for:
/// - Sliding tiles in the specified direction
/// - Merging identical adjacent tiles
/// - Detecting whether any movement occurred
///
/// WHY BENCHMARK THIS:
/// - Called on every user input (arrow key press)
/// - Most computationally complex operation in the game
/// - Performance directly affects game responsiveness
/// - Algorithm complexity varies significantly with grid density
///
/// BENCHMARKING STRATEGY:
/// We test two extreme scenarios to understand performance bounds:
/// 1. Empty grid: Minimal computational work (early exit conditions)
/// 2. Full grid: Maximum computational work (all cells need processing)
///
/// POTENTIAL IMPROVEMENTS:
/// - Could test all four directions (Up, Down, Left, Right)
/// - Missing intermediate grid densities (25%, 50%, 75% full)
/// - No testing of merge-heavy scenarios vs slide-heavy scenarios
/// - Could benchmark with realistic game states (saved from actual gameplay)
fn benchmark_move_tiles(c: &mut Criterion) {
    let mut group = c.benchmark_group("move_tiles");

    // Set up test data: empty grid (worst case for early exit optimization)
    let mut empty_state = GameState::new();
    // Clear the initial tiles that GameState::new() adds by default
    empty_state.grid = [[0; GRID_SIZE as usize]; GRID_SIZE as usize];

    // Set up test data: completely full grid (worst case for computational complexity)
    let mut full_state = GameState::new();
    // Fill every cell with the value 2 (creates maximum merging opportunities)
    for i in 0..GRID_SIZE as usize {
        for j in 0..GRID_SIZE as usize {
            full_state.grid[i][j] = 2;
        }
    }

    // Benchmark 1: Movement on empty grid
    // Expected: Very fast execution due to early exit conditions
    // This measures the overhead of the movement algorithm when no work is needed
    group.bench_function("move_right_empty_state", |b| {
        b.iter(|| empty_state.move_tiles(black_box(Direction::Right)))
    });

    // Benchmark 2: Movement on completely full grid
    // Expected: Slower execution as algorithm must check every cell
    // This measures maximum computational load of the movement algorithm
    group.bench_function("move_right_full_state", |b| {
        b.iter(|| full_state.move_tiles(black_box(Direction::Right)))
    });

    group.finish();
}

/// Benchmarks the game over detection algorithm under different grid conditions
///
/// WHAT IS BEING BENCHMARKED:
/// The check_game_over() method, which determines if the game has ended by:
/// - Checking if any empty cells exist (quick path to determine game continues)
/// - If grid is full, checking if any valid moves remain (slower path)
/// - Validating potential merges in all four directions
///
/// WHY BENCHMARK THIS:
/// - Called after every move to update game state
/// - Algorithm complexity increases dramatically with grid density
/// - Performance affects responsiveness when checking end-game conditions
/// - Different grid patterns have vastly different computational requirements
///
/// BENCHMARKING STRATEGY:
/// We test the two primary execution paths:
/// 1. Empty grid: Fast path (immediate return due to available moves)
/// 2. Full grid: Slow path (must check all adjacent cell pairs for merges)
///
/// POTENTIAL IMPROVEMENTS:
/// - Missing "near full" scenarios (1-2 empty cells) which are common in real games
/// - Full grid uses alternating pattern but only tests one specific layout
/// - Could test grids where moves are available vs unavailable separately
/// - No testing of scenarios with high merge potential vs no merge potential
fn benchmark_game_over(c: &mut Criterion) {
    let mut group = c.benchmark_group("game_over");

    // Set up test data: empty grid (fast path - game definitely not over)
    let mut empty_state = GameState::new();
    // Clear initial tiles to create truly empty grid
    empty_state.grid = [[0; GRID_SIZE as usize]; GRID_SIZE as usize];

    // Set up test data: full grid with alternating pattern (slow path - must check all merges)
    let mut full_state = GameState::new();
    // Create alternating 2s and 4s pattern which prevents most merges
    // This forces the algorithm to check every adjacent pair without finding valid moves
    for i in 0..GRID_SIZE as usize {
        for j in 0..GRID_SIZE as usize {
            full_state.grid[i][j] = if (i + j) % 2 == 0 { 2 } else { 4 };
        }
    }

    // Benchmark 1: Game over check on empty grid
    // Expected: Very fast execution (immediate return - game not over)
    // This measures the best-case performance when empty cells exist
    group.bench_function("game_over_empty_state", |b| {
        b.iter(|| empty_state.check_game_over())
    });

    // Benchmark 2: Game over check on full alternating grid
    // Expected: Slower execution (must verify no moves available)
    // This measures worst-case performance when extensive checking is required
    group.bench_function("game_over_full_state", |b| {
        b.iter(|| full_state.check_game_over())
    });

    group.finish();
}

criterion_group!(benches, benchmark_move_tiles, benchmark_game_over);
criterion_main!(benches);
