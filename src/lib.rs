// 2048 Game Library - Core Game Logic and Rendering
//
// This library implements the complete 2048 game logic including:
// - Grid management and tile movement algorithms
// - Game state tracking and win/lose conditions
// - Random tile generation with weighted probability
// - Visual rendering with ggez graphics framework
// - Input handling for arrow key controls

use ggez::{
    event,
    graphics::{self, Color, DrawParam, Rect, Text},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};
use rand::seq::SliceRandom;
use std::collections::HashMap;

// === GAME CONSTANTS ===
// These constants define the visual layout and game parameters

/// Total window size in pixels (creates a square window)
pub const WINDOW_SIZE: f32 = 600.0;

/// Grid dimensions (4x4 grid for classic 2048 gameplay)
pub const GRID_SIZE: u32 = 4;

/// Size of each individual cell in pixels (calculated to fit the window perfectly)
pub const CELL_SIZE: f32 = WINDOW_SIZE / GRID_SIZE as f32;

/// Padding between cells in pixels (creates visual separation between tiles)
pub const PADDING: f32 = 10.0;

// === GAME STATE STRUCTURE ===

/// Main game state structure that holds all game data and implements the game loop
///
/// This struct manages:
/// - The 4x4 grid of tile values (0 represents empty cells)
/// - Color mapping for different tile values
/// - Game over state tracking
/// - All game logic through method implementations
pub struct GameState {
    /// 2D array representing the game grid, where each cell contains a tile value
    /// Value 0 represents an empty cell, powers of 2 (2, 4, 8, 16, ...) represent tiles
    pub grid: [[u32; GRID_SIZE as usize]; GRID_SIZE as usize],

    /// HashMap mapping tile values to their corresponding colors for rendering
    /// This allows easy lookup of colors based on tile values during drawing
    colors: HashMap<u32, Color>,

    /// Boolean flag indicating whether the game has ended (no moves available)
    game_over: bool,
}

// === GAME STATE IMPLEMENTATION ===

impl GameState {
    /// Creates a new game state with initialized colors and starting tiles
    ///
    /// This constructor:
    /// 1. Initializes an empty 4x4 grid (all zeros)
    /// 2. Sets up the color palette for different tile values
    /// 3. Adds two random starting tiles to begin the game
    ///
    /// # Returns
    ///
    /// * `Self` - A fully initialized GameState ready to play
    pub fn new() -> Self {
        let mut state = GameState {
            grid: [[0; GRID_SIZE as usize]; GRID_SIZE as usize],
            colors: HashMap::new(),
            game_over: false,
        };

        // Initialize color palette for tile visualization
        // Colors progress from light (low values) to vibrant (high values)
        // This creates a visual hierarchy that helps players identify tile values
        state.colors.insert(0, Color::from_rgb(205, 193, 180)); // Empty cell - neutral gray
        state.colors.insert(2, Color::from_rgb(238, 228, 218)); // 2 - light beige
        state.colors.insert(4, Color::from_rgb(237, 224, 200)); // 4 - slightly darker beige
        state.colors.insert(8, Color::from_rgb(242, 177, 121)); // 8 - light orange
        state.colors.insert(16, Color::from_rgb(245, 149, 99)); // 16 - medium orange
        state.colors.insert(32, Color::from_rgb(246, 124, 95)); // 32 - darker orange
        state.colors.insert(64, Color::from_rgb(246, 94, 59)); // 64 - red-orange
        state.colors.insert(128, Color::from_rgb(237, 207, 114)); // 128 - light yellow
        state.colors.insert(256, Color::from_rgb(237, 204, 97)); // 256 - medium yellow
        state.colors.insert(512, Color::from_rgb(237, 200, 80)); // 512 - darker yellow
        state.colors.insert(1024, Color::from_rgb(237, 197, 63)); // 1024 - gold
        state.colors.insert(2048, Color::from_rgb(237, 194, 46)); // 2048 - bright gold (victory!)

        // Add two initial tiles to start the game
        // Standard 2048 gameplay begins with two tiles on the board
        state.add_random_tile();
        state.add_random_tile();

        state
    }

    // === TILE GENERATION ===

    /// Adds a random tile (2 or 4) to a random empty cell on the grid
    ///
    /// This function implements the core tile spawning mechanism of 2048:
    /// 1. Finds all empty cells (value 0) on the grid
    /// 2. Randomly selects one empty cell
    /// 3. Places either a 2 (90% chance) or 4 (10% chance) in that cell
    ///
    /// The 90/10 probability split ensures that 2s are more common than 4s,
    /// which maintains game balance and prevents the board from filling too quickly.
    ///
    /// # Behavior
    ///
    /// * Does nothing if no empty cells are available
    /// * Uses thread-local random number generator for randomness
    pub fn add_random_tile(&mut self) {
        let mut empty_cells = Vec::new();

        // Scan the entire grid to find all empty cells (cells with value 0)
        for i in 0..GRID_SIZE as usize {
            for j in 0..GRID_SIZE as usize {
                if self.grid[i][j] == 0 {
                    empty_cells.push((i, j));
                }
            }
        }

        // If there are empty cells available, place a new tile randomly
        if let Some(&(x, y)) = empty_cells.choose(&mut rand::thread_rng()) {
            // Use weighted probability: 90% chance for 2, 10% chance for 4
            // This matches the original 2048 game's spawn mechanics
            self.grid[x][y] = if rand::random::<f32>() < 0.9 { 2 } else { 4 };
        }
    }

    // === MOVEMENT LOGIC ===

    /// Central movement dispatcher that handles tile movement in any direction
    ///
    /// This function serves as the main entry point for all tile movements.
    /// It delegates to specific directional movement functions based on the
    /// direction parameter.
    ///
    /// # Arguments
    ///
    /// * `direction` - The direction to move tiles (Up, Down, Left, Right)
    ///
    /// # Returns
    ///
    /// * `bool` - True if any tiles moved, false if no movement occurred
    ///           This is used to determine if a new tile should be spawned
    pub fn move_tiles(&mut self, direction: Direction) -> bool {
        match direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }

    /// Moves all tiles to the right and merges identical adjacent tiles
    ///
    /// This function implements the core 2048 movement algorithm for rightward movement:
    /// 1. Processes each row from right to left (reverse order)
    /// 2. For each non-empty tile, slides it as far right as possible
    /// 3. Merges tiles with identical values when they collide
    /// 4. Ensures each tile can only merge once per move
    ///
    /// # Algorithm Details
    ///
    /// The algorithm uses a "merged" tracking array to prevent tiles from merging
    /// multiple times in a single move, which is crucial for correct 2048 gameplay.
    ///
    /// # Returns
    ///
    /// * `bool` - True if any tiles moved or merged, false otherwise
    pub fn move_right(&mut self) -> bool {
        let mut moved = false;
        // Track which cells have already merged this turn to prevent double-merging
        let mut merged = [[false; GRID_SIZE as usize]; GRID_SIZE as usize];

        // Process each row
        for i in 0..GRID_SIZE as usize {
            // Process columns from right to left (reverse order)
            // This ensures tiles slide as far right as possible
            for j in (0..GRID_SIZE as usize - 1).rev() {
                if self.grid[i][j] != 0 {
                    let mut col = j;

                    // Slide the tile as far right as possible
                    while col < GRID_SIZE as usize - 1 {
                        // Case 1: Empty cell to the right - slide the tile
                        if self.grid[i][col + 1] == 0 {
                            self.grid[i][col + 1] = self.grid[i][col];
                            self.grid[i][col] = 0;
                            moved = true;
                            col += 1;
                        }
                        // Case 2: Matching tile to the right that hasn't merged yet - merge them
                        else if self.grid[i][col + 1] == self.grid[i][col] && !merged[i][col + 1]
                        {
                            self.grid[i][col + 1] *= 2; // Double the value
                            self.grid[i][col] = 0; // Remove the original tile
                            merged[i][col + 1] = true; // Mark as merged to prevent double-merging
                            moved = true;
                            break; // Stop sliding this tile
                        }
                        // Case 3: Different tile or already merged - stop sliding
                        else {
                            break;
                        }
                    }
                }
            }
        }
        moved
    }

    /// Moves all tiles to the left and merges identical adjacent tiles
    ///
    /// Implements the same sliding and merging algorithm as move_right()
    /// but processes columns from left to right instead.
    pub fn move_left(&mut self) -> bool {
        let mut moved = false;
        let mut merged = [[false; GRID_SIZE as usize]; GRID_SIZE as usize];

        // move left
        for i in 0..GRID_SIZE as usize {
            for j in 1..GRID_SIZE as usize {
                // if the cell is not empty
                if self.grid[i][j] != 0 {
                    let mut col = j;
                    // move left
                    while col > 0 {
                        // if the cell to the left is empty
                        if self.grid[i][col - 1] == 0 {
                            self.grid[i][col - 1] = self.grid[i][col]; // move the tile to the left
                            self.grid[i][col] = 0; // set the current cell to 0
                            moved = true; // set the moved flag to true
                            col -= 1; // move the column to the left
                        }
                        // merge tiles
                        else if self.grid[i][col - 1] == self.grid[i][col] && !merged[i][col - 1]
                        // if the cell to the left is not merged
                        {
                            self.grid[i][col - 1] *= 2; // merge tiles
                            self.grid[i][col] = 0; // set the current cell to 0
                            merged[i][col - 1] = true; // set the merged cell to true
                            moved = true; // set the moved flag to true
                            break; // break the loop
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        moved
    }

    /// Moves all tiles up and merges identical adjacent tiles
    ///
    /// Implements the same sliding and merging algorithm as move_right()
    /// but processes rows from top to bottom instead.
    pub fn move_up(&mut self) -> bool {
        let mut moved = false;
        let mut merged = [[false; GRID_SIZE as usize]; GRID_SIZE as usize];

        for j in 0..GRID_SIZE as usize {
            for i in 1..GRID_SIZE as usize {
                // if the cell is not empty
                if self.grid[i][j] != 0 {
                    let mut row = i;
                    // move up
                    while row > 0 {
                        // if the cell above is empty
                        if self.grid[row - 1][j] == 0 {
                            // move the tile up
                            self.grid[row - 1][j] = self.grid[row][j];
                            self.grid[row][j] = 0; // set the current cell to 0
                            moved = true; // set the moved flag to true
                            row -= 1; // move the row up
                        }
                        // merge tiles
                        else if self.grid[row - 1][j] == self.grid[row][j] && !merged[row - 1][j]
                        {
                            self.grid[row - 1][j] *= 2; // merge tiles
                            self.grid[row][j] = 0; // set the current cell to 0
                            merged[row - 1][j] = true; // set the merged cell to true
                            moved = true; // set the moved flag to true
                            break; // break the loop
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        moved
    }

    /// Moves all tiles down and merges identical adjacent tiles
    ///
    /// Implements the same sliding and merging algorithm as move_right()
    /// but processes rows from bottom to top instead.
    pub fn move_down(&mut self) -> bool {
        let mut moved = false;
        let mut merged = [[false; GRID_SIZE as usize]; GRID_SIZE as usize];

        for j in 0..GRID_SIZE as usize {
            for i in (0..GRID_SIZE as usize - 1).rev() {
                // if the cell is not empty
                if self.grid[i][j] != 0 {
                    let mut row = i;
                    // move down
                    while row < GRID_SIZE as usize - 1 {
                        // if the cell below is empty
                        if self.grid[row + 1][j] == 0 {
                            self.grid[row + 1][j] = self.grid[row][j]; // move the tile down
                            self.grid[row][j] = 0; // set the current cell to 0
                            moved = true; // set the moved flag to true
                            row += 1; // move the row down
                        }
                        // merge tiles
                        else if self.grid[row + 1][j] == self.grid[row][j] && !merged[row + 1][j]
                        {
                            self.grid[row + 1][j] *= 2; // merge tiles
                            self.grid[row][j] = 0; // set the current cell to 0
                            merged[row + 1][j] = true; // set the merged cell to true
                            moved = true; // set the moved flag to true
                            break; // break the loop
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        moved
    }

    // === GAME STATE CHECKING ===

    /// Determines if any moves are still possible on the current board
    ///
    /// This function checks for game over conditions by examining:
    /// 1. Whether any empty cells exist (if so, moves are always possible)
    /// 2. Whether any adjacent tiles have matching values (merging is possible)
    ///
    /// # Algorithm Optimization
    ///
    /// Only checks right and down directions for each cell, which is sufficient
    /// since adjacency is symmetric. This halves the number of comparisons needed.
    ///
    /// # Returns
    ///
    /// * `bool` - True if moves are available, false if the game is stuck
    pub fn has_moves_available(&self) -> bool {
        for i in 0..GRID_SIZE as usize {
            for j in 0..GRID_SIZE as usize {
                // If any cell is empty, moves are definitely available
                if self.grid[i][j] == 0 {
                    return true;
                }

                let current = self.grid[i][j];

                // Check if current tile can merge with the tile to its right
                if j < GRID_SIZE as usize - 1 && current == self.grid[i][j + 1] {
                    return true;
                }

                // Check if current tile can merge with the tile below it
                if i < GRID_SIZE as usize - 1 && current == self.grid[i + 1][j] {
                    return true;
                }
            }
        }
        false
    }

    /// Checks if the game is over (no moves available)
    ///
    /// This is a simple wrapper around has_moves_available() that inverts the result.
    /// The game is over when no moves are available.
    ///
    /// # Returns
    ///
    /// * `bool` - True if the game is over, false if moves are still possible
    pub fn check_game_over(&self) -> bool {
        !self.has_moves_available()
    }

    /// Resets the game to its initial state
    ///
    /// This function:
    /// 1. Clears the entire grid (sets all cells to 0)
    /// 2. Resets the game_over flag to false
    /// 3. Adds two random starting tiles
    ///
    /// Used when the player presses Enter after a game over to start a new game.
    pub fn restart_game(&mut self) {
        // Clear the grid
        self.grid = [[0; GRID_SIZE as usize]; GRID_SIZE as usize];
        self.game_over = false;

        // Add starting tiles for the new game
        self.add_random_tile();
        self.add_random_tile();
    }
}

// === DIRECTION ENUM ===

/// Represents the four possible movement directions in 2048
///
/// This enum is used to specify which direction tiles should move
/// when the player presses arrow keys. Each variant corresponds to
/// one of the four movement functions in GameState.
pub enum Direction {
    /// Move tiles upward (arrow key up)
    Up,
    /// Move tiles downward (arrow key down)
    Down,
    /// Move tiles leftward (arrow key left)
    Left,
    /// Move tiles rightward (arrow key right)
    Right,
}

// === EVENT HANDLER IMPLEMENTATION ===

/// Implementation of ggez's EventHandler trait for GameState
///
/// This implementation handles the main game loop events:
/// - update(): Called every frame for game logic updates
/// - draw(): Called every frame to render the game
/// - key_down_event(): Called when keys are pressed for input handling
impl event::EventHandler<ggez::GameError> for GameState {
    /// Updates game state each frame
    ///
    /// Currently does nothing since 2048 is turn-based and only changes
    /// state in response to input. In a real-time game, this would contain
    /// animation updates, AI logic, etc.
    ///
    /// # Arguments
    ///
    /// * `_ctx` - The ggez context (unused in this simple game)
    ///
    /// # Returns
    ///
    /// * `GameResult` - Always returns Ok(()) for this game
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Renders the entire game screen
    ///
    /// This function handles all visual rendering including:
    /// 1. Grid background and individual cell backgrounds
    /// 2. Tile colors based on their values
    /// 3. Numbers displayed on each tile
    /// 4. Game over overlay with restart instructions
    ///
    /// # Rendering Process
    ///
    /// 1. Creates a canvas with the background color
    /// 2. Draws each cell as a colored rectangle
    /// 3. Draws tile numbers with appropriate text color
    /// 4. Overlays game over screen if applicable
    ///
    /// # Arguments
    ///
    /// * `ctx` - The ggez graphics context for rendering operations
    ///
    /// # Returns
    ///
    /// * `GameResult` - Ok(()) on successful render, or graphics error
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Create a canvas with the game's background color (warm beige)
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(187, 173, 160));

        // === GRID RENDERING ===
        // Draw each cell in the 4x4 grid
        for i in 0..GRID_SIZE as usize {
            for j in 0..GRID_SIZE as usize {
                let cell_value = self.grid[i][j];

                // Look up the color for this tile value from our color palette
                let color = self.colors.get(&cell_value).unwrap_or(&Color::WHITE);

                // Calculate cell position and size with padding for visual separation
                let rect = Rect::new(
                    j as f32 * CELL_SIZE + PADDING, // x position
                    i as f32 * CELL_SIZE + PADDING, // y position
                    CELL_SIZE - PADDING * 2.0,      // width (reduced by padding on both sides)
                    CELL_SIZE - PADDING * 2.0,      // height (reduced by padding on both sides)
                );

                // Draw the cell background as a filled rectangle
                canvas.draw(
                    &graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, *color)?,
                    DrawParam::default(),
                );

                // === TEXT RENDERING ===
                // Only draw numbers on non-empty cells
                if cell_value != 0 {
                    let mut text = Text::new(format!("{}", cell_value));
                    text.set_scale(50.0);

                    // Choose text color for readability based on tile value
                    // Low values (2, 4) use dark text, higher values use white text
                    let text_color = if cell_value <= 4 {
                        Color::from_rgb(119, 110, 101) // Dark gray for light backgrounds
                    } else {
                        Color::WHITE // White for darker backgrounds
                    };

                    // Draw the text centered in the cell
                    canvas.draw(
                        &text,
                        DrawParam::default()
                            .color(text_color)
                            .dest([
                                j as f32 * CELL_SIZE + CELL_SIZE / 2.0, // Center horizontally
                                i as f32 * CELL_SIZE + CELL_SIZE / 2.0, // Center vertically
                            ])
                            .offset([0.5, 0.5]), // Center the text anchor point
                    );
                }
            }
        }

        // === GAME OVER OVERLAY ===
        // Draw semi-transparent overlay and instructions when game ends
        if self.game_over {
            // Create a semi-transparent black overlay covering the entire screen
            // This dims the game board and draws attention to the game over message
            let overlay = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0.0, 0.0, WINDOW_SIZE, WINDOW_SIZE),
                Color::from_rgba(0, 0, 0, 180), // Black with ~70% transparency
            )?;
            canvas.draw(&overlay, DrawParam::default());

            // Create and style the main game over message
            let mut game_over_text = Text::new("Game Over!");
            game_over_text.set_scale(80.0);

            // Create and style the restart instruction
            let mut restart_text = Text::new("Press Enter to restart");
            restart_text.set_scale(40.0);

            // Draw the game over message centered on screen, slightly above center
            canvas.draw(
                &game_over_text,
                DrawParam::default()
                    .color(Color::WHITE)
                    .dest([WINDOW_SIZE / 2.0, WINDOW_SIZE / 2.0 - 50.0])
                    .offset([0.5, 0.5]), // Center the text anchor
            );

            // Draw the restart instruction centered on screen, slightly below center
            canvas.draw(
                &restart_text,
                DrawParam::default()
                    .color(Color::WHITE)
                    .dest([WINDOW_SIZE / 2.0, WINDOW_SIZE / 2.0 + 50.0])
                    .offset([0.5, 0.5]), // Center the text anchor
            );
        }

        // Finalize and present the rendered frame
        canvas.finish(ctx)?;
        Ok(())
    }

    /// Handles keyboard input for game controls
    ///
    /// This function processes two types of input:
    /// 1. During gameplay: Arrow keys for tile movement
    /// 2. During game over: Enter key to restart the game
    ///
    /// # Game Logic Flow
    ///
    /// When a movement key is pressed:
    /// 1. Attempt to move tiles in the specified direction
    /// 2. If any tiles moved, spawn a new random tile
    /// 3. Check if the game is over (no moves available)
    /// 4. Update game state accordingly
    ///
    /// # Arguments
    ///
    /// * `_ctx` - The ggez context (unused)
    /// * `key` - The key input event containing keycode information
    /// * `_repeat` - Whether this is a repeated key press (ignored)
    ///
    /// # Returns
    ///
    /// * `GameResult` - Always returns Ok(()) for this game
    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyInput, _repeat: bool) -> GameResult {
        if let Some(keycode) = key.keycode {
            // === GAME OVER STATE HANDLING ===
            if self.game_over {
                // When game is over, only the Enter key is functional (for restart)
                if keycode == KeyCode::Return {
                    self.restart_game();
                }
                return Ok(());
            }

            // === MOVEMENT INPUT MAPPING ===
            // Map arrow keys to movement directions
            let direction = match keycode {
                KeyCode::Up => Some(Direction::Up),
                KeyCode::Down => Some(Direction::Down),
                KeyCode::Left => Some(Direction::Left),
                KeyCode::Right => Some(Direction::Right),
                _ => None, // Ignore all other keys during gameplay
            };

            // === GAME LOGIC EXECUTION ===
            // Process the movement if a valid direction was pressed
            if let Some(direction) = direction {
                // Only proceed if tiles actually moved (prevents unnecessary tile spawning)
                if self.move_tiles(direction) {
                    // Spawn a new tile after successful movement
                    self.add_random_tile();

                    // Check if the game should end
                    // First, quickly check if there are any empty cells
                    let mut has_empty = false;
                    'outer: for row in &self.grid {
                        for &cell in row {
                            if cell == 0 {
                                has_empty = true;
                                break 'outer;
                            }
                        }
                    }

                    // Only run the expensive game over check if the grid is full
                    // (if there are empty cells, the game definitely isn't over)
                    if !has_empty && self.check_game_over() {
                        self.game_over = true;
                    }
                }
            }
        }

        Ok(())
    }
}
