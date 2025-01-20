use ggez::{conf, event, ContextBuilder, GameResult};
use rust_2048_game::GameState;

fn main() -> GameResult {
    // Create a context builder
    let cb = ContextBuilder::new("2048", "ggez")
        .window_setup(conf::WindowSetup::default().title("2048"))
        .window_mode(conf::WindowMode::default().dimensions(
            rust_2048_game::GRID_SIZE as f32 * rust_2048_game::CELL_SIZE,
            rust_2048_game::GRID_SIZE as f32 * rust_2048_game::CELL_SIZE,
        ));

    // Build the context
    let (ctx, event_loop) = cb.build()?;

    // Create the game state
    let state = GameState::new();

    // Run the game
    event::run(ctx, event_loop, state)
}
