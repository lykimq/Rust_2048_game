// 2048 Game - Main Entry Point
//
// This is the main entry point for the 2048 game written in Rust using the ggez game framework.
// The game implements the classic 2048 sliding puzzle game where the player combines numbered
// tiles to reach the 2048 tile.

use ggez::{conf, event, ContextBuilder, GameResult};
use rust_2048_game::GameState;

/// Main function that initializes and runs the 2048 game
///
/// This function performs the following tasks:
/// 1. Creates a ggez context with window configuration
/// 2. Sets up the game window with appropriate dimensions
/// 3. Initializes the game state with a fresh grid
/// 4. Starts the main game loop
///
/// # Returns
///
/// * `GameResult` - Returns Ok(()) on successful game completion or an error if initialization fails
fn main() -> GameResult {
    // Create a context builder with game name and author
    // ggez uses this information for window management and debugging
    let cb = ContextBuilder::new("2048", "ggez")
        // Configure the window title that appears in the title bar
        .window_setup(conf::WindowSetup::default().title("2048"))
        // Set window dimensions based on grid size and cell size
        // This ensures the window is perfectly sized for our 4x4 grid
        .window_mode(conf::WindowMode::default().dimensions(
            rust_2048_game::GRID_SIZE as f32 * rust_2048_game::CELL_SIZE,
            rust_2048_game::GRID_SIZE as f32 * rust_2048_game::CELL_SIZE,
        ));

    // Build the graphics context and event loop from the configuration
    // The context handles rendering and the event loop manages input/update cycles
    let (ctx, event_loop) = cb.build()?;

    // Initialize the game state with an empty grid and add two starting tiles
    // GameState::new() sets up the initial game board with two random tiles (2 or 4)
    let state = GameState::new();

    // Start the main game loop using ggez's event system
    // This will call our update() and draw() methods repeatedly until the game exits
    event::run(ctx, event_loop, state)
}
