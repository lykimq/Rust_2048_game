use ggez::{
    event,
    graphics::{self, Color, DrawParam, Rect, Text},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};
use rand::seq::SliceRandom;
use std::collections::HashMap;

pub const WINDOW_SIZE: f32 = 600.0;
pub const GRID_SIZE: u32 = 4;
pub const CELL_SIZE: f32 = WINDOW_SIZE / GRID_SIZE as f32;
pub const PADDING: f32 = 10.0;

pub struct GameState {
    pub grid: [[u32; GRID_SIZE as usize]; GRID_SIZE as usize],
    colors: HashMap<u32, Color>,
    game_over: bool,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = GameState {
            grid: [[0; GRID_SIZE as usize]; GRID_SIZE as usize],
            colors: HashMap::new(),
            game_over: false,
        };

        // Initialize colors for different numbers
        state.colors.insert(0, Color::from_rgb(205, 193, 180)); // Empty cell
        state.colors.insert(2, Color::from_rgb(238, 228, 218)); // 2
        state.colors.insert(4, Color::from_rgb(237, 224, 200)); // 4
        state.colors.insert(8, Color::from_rgb(242, 177, 121));
        state.colors.insert(16, Color::from_rgb(245, 149, 99)); // 16
        state.colors.insert(32, Color::from_rgb(246, 124, 95)); // 32
        state.colors.insert(64, Color::from_rgb(246, 94, 59)); // 64
        state.colors.insert(128, Color::from_rgb(237, 207, 114));
        state.colors.insert(256, Color::from_rgb(237, 204, 97));
        state.colors.insert(512, Color::from_rgb(237, 200, 80));
        state.colors.insert(1024, Color::from_rgb(237, 197, 63));
        state.colors.insert(2048, Color::from_rgb(237, 194, 46));

        // Add initial tiles

        state.add_random_tile();
        state.add_random_tile();

        state
    }

    // Add random tile to the grid
    pub fn add_random_tile(&mut self) {
        let mut empty_cells = Vec::new();
        // Get all the empty cells
        for i in 0..GRID_SIZE as usize {
            // Get all the empty cells in the row
            for j in 0..GRID_SIZE as usize {
                // If the cell is empty
                if self.grid[i][j] == 0 {
                    // Add the cell to the empty cells vector
                    empty_cells.push((i, j));
                }
            }
        }

        // Add a 2 or 4 tile to a random empty cell
        if let Some(&(x, y)) = empty_cells.choose(&mut rand::thread_rng()) {
            // Add a 2 or 4 tile to the random empty cell
            self.grid[x][y] = if rand::random::<f32>() < 0.9 { 2 } else { 4 };
        }
    }

    // Move tiles in a direction
    pub fn move_tiles(&mut self, direction: Direction) -> bool {
        match direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }

    // move right
    pub fn move_right(&mut self) -> bool {
        let mut moved = false;
        let mut merged = [[false; GRID_SIZE as usize]; GRID_SIZE as usize];

        for i in 0..GRID_SIZE as usize {
            for j in (0..GRID_SIZE as usize - 1).rev() {
                // if the cell is not empty
                if self.grid[i][j] != 0 {
                    let mut col = j;
                    // move right
                    while col < GRID_SIZE as usize - 1 {
                        // if the cell to the right is empty
                        if self.grid[i][col + 1] == 0 {
                            self.grid[i][col + 1] = self.grid[i][col]; // move the tile to the right
                            self.grid[i][col] = 0; // set the current cell to 0
                            moved = true; // set the moved flag to true
                            col += 1; // move the column to the right
                        }
                        // merge tiles
                        else if self.grid[i][col + 1] == self.grid[i][col] && !merged[i][col + 1]
                        {
                            self.grid[i][col + 1] *= 2; // merge tiles
                            self.grid[i][col] = 0; // set the current cell to 0
                            merged[i][col + 1] = true; // set the merged cell to true
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

    // move left
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

    // move up
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

    // move down
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

    // Check moves availability
    pub fn has_moves_available(&self) -> bool {
        for i in 0..GRID_SIZE as usize {
            for j in 0..GRID_SIZE as usize {
                // If we find an empty cell, moves are available
                if self.grid[i][j] == 0 {
                    return true;
                }

                let current = self.grid[i][j];
                // Only check right and down (no need to check both directions)
                // This halves the number of comparisons needed
                if j < GRID_SIZE as usize - 1 && current == self.grid[i][j + 1] {
                    return true;
                }
                if i < GRID_SIZE as usize - 1 && current == self.grid[i + 1][j] {
                    return true;
                }
            }
        }
        false
    }

    // Check if the game is over
    pub fn check_game_over(&self) -> bool {
        !self.has_moves_available()
    }

    // Restart the game
    pub fn restart_game(&mut self) {
        self.grid = [[0; GRID_SIZE as usize]; GRID_SIZE as usize];
        self.game_over = false;
        self.add_random_tile();
        self.add_random_tile();
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    // Draw the game
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Create a canvas
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(187, 173, 160));

        // Draw grid
        for i in 0..GRID_SIZE as usize {
            // Draw each row
            for j in 0..GRID_SIZE as usize {
                // Get the cell value and color
                let cell_value = self.grid[i][j];

                // Get the color from the colors map
                let color = self.colors.get(&cell_value).unwrap_or(&Color::WHITE);

                // Draw the cell
                let rect = Rect::new(
                    j as f32 * CELL_SIZE + PADDING, // x
                    i as f32 * CELL_SIZE + PADDING, // y
                    CELL_SIZE - PADDING * 2.0,      // width
                    CELL_SIZE - PADDING * 2.0,      // height
                );

                // Draw the rectangle
                canvas.draw(
                    &graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, *color)?,
                    DrawParam::default(),
                );

                // Draw the text
                if cell_value != 0 {
                    let mut text = Text::new(format!("{}", cell_value));
                    text.set_scale(50.0);

                    // Set the text color based on the cell value
                    let text_color = if cell_value <= 4 {
                        Color::from_rgb(119, 110, 101)
                    } else {
                        Color::WHITE
                    };

                    // Draw the text
                    canvas.draw(
                        &text,
                        DrawParam::default()
                            .color(text_color)
                            .dest([
                                j as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                                i as f32 * CELL_SIZE + CELL_SIZE / 2.0,
                            ])
                            .offset([0.5, 0.5]),
                    );
                }
            }
        }

        // Draw game over overlay if game is over
        if self.game_over {
            // Create full-screen semi-transparent overlay
            let overlay = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0.0, 0.0, WINDOW_SIZE, WINDOW_SIZE),
                Color::from_rgba(0, 0, 0, 180),
            )?;
            canvas.draw(&overlay, DrawParam::default());

            // Draw game over message
            let mut game_over_text = Text::new("Game Over!");
            game_over_text.set_scale(80.0);

            // Draw restart instruction
            let mut restart_text = Text::new("Press Enter to restart");
            restart_text.set_scale(40.0);

            // Center the text on screen
            canvas.draw(
                &game_over_text,
                DrawParam::default()
                    .color(Color::WHITE)
                    .dest([WINDOW_SIZE / 2.0, WINDOW_SIZE / 2.0 - 50.0])
                    .offset([0.5, 0.5]),
            );

            // Draw restart instruction
            canvas.draw(
                &restart_text,
                DrawParam::default()
                    .color(Color::WHITE)
                    .dest([WINDOW_SIZE / 2.0, WINDOW_SIZE / 2.0 + 50.0])
                    .offset([0.5, 0.5]),
            );
        }

        // Finish drawing
        canvas.finish(ctx)?;
        Ok(())
    }

    // Handle key presses
    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyInput, _repeat: bool) -> GameResult {
        if let Some(keycode) = key.keycode {
            if self.game_over {
                // If game is over, only respond to Enter/Return key to restart
                if keycode == KeyCode::Return {
                    self.restart_game();
                }
                return Ok(());
            }

            // Handle movement keys
            let direction = match keycode {
                KeyCode::Up => Some(Direction::Up),
                KeyCode::Down => Some(Direction::Down),
                KeyCode::Left => Some(Direction::Left),
                KeyCode::Right => Some(Direction::Right),
                _ => None,
            };

            // Only check for game over if a move actually happened
            if let Some(direction) = direction {
                if self.move_tiles(direction) {
                    self.add_random_tile();
                    // Only check for game over if the grid is full
                    let mut has_empty = false;
                    'outer: for row in &self.grid {
                        for &cell in row {
                            if cell == 0 {
                                has_empty = true;
                                break 'outer;
                            }
                        }
                    }
                    if !has_empty && self.check_game_over() {
                        self.game_over = true;
                    }
                }
            }
        }

        Ok(())
    }
}
