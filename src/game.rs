use ggez::{Context, GameResult};
use ggez::graphics;

use crate::tetromino::Tetromino;
use crate::board::Board;
use crate::animations::AnimationManager;

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}

pub struct Game {
    pub board: Board,
    pub current_tetromino: Option<Tetromino>,
    pub next_tetromino: Tetromino,
    pub score: u32,
    pub level: u32,
    pub lines_cleared: u32,
    pub state: GameState,
    pub drop_timer: f32,
    pub drop_interval: f32,
    pub block_size: f32,
    pub animations: AnimationManager,
}

impl Game {
    pub fn new() -> Self {
        let block_size = 25.0;
        let offset_x = 50.0;
        let offset_y = 50.0;
        
        Game {
            board: Board::new(block_size, offset_x, offset_y),
            current_tetromino: None,
            next_tetromino: Tetromino::random(),
            score: 0,
            level: 1,
            lines_cleared: 0,
            state: GameState::Menu,
            drop_timer: 0.0,
            drop_interval: 1.0,
            block_size,
            animations: AnimationManager::new(),
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        if self.state != GameState::Playing {
            return;
        }

        let dt = ctx.time.delta().as_secs_f32();
        self.drop_timer += dt;

        if self.drop_timer >= self.drop_interval {
            self.drop_timer = 0.0;
            self.move_down();
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        // Draw board
        self.board.draw(ctx, canvas)?;

        // Draw current tetromino
        if let Some(ref tetromino) = self.current_tetromino {
            // Draw ghost piece
            self.board.draw_ghost(ctx, canvas, tetromino)?;
            
            // Draw current tetromino
            tetromino.draw(ctx, canvas, self.block_size, self.board.offset_x, self.board.offset_y)?;
        }

        Ok(())
    }

    pub fn spawn_tetromino(&mut self) {
        self.current_tetromino = Some(self.next_tetromino.clone());
        self.next_tetromino = Tetromino::random();
        
        // Check if game is over
        if let Some(ref tetromino) = self.current_tetromino {
            if !self.board.is_valid_position(tetromino) {
                self.state = GameState::GameOver;
            }
        }
    }

    pub fn move_left(&mut self) {
        if self.state != GameState::Playing {
            return;
        }

        if let Some(ref mut tetromino) = self.current_tetromino {
            tetromino.x -= 1;
            if !self.board.is_valid_position(tetromino) {
                tetromino.x += 1;
            }
        }
    }

    pub fn move_right(&mut self) {
        if self.state != GameState::Playing {
            return;
        }

        if let Some(ref mut tetromino) = self.current_tetromino {
            tetromino.x += 1;
            if !self.board.is_valid_position(tetromino) {
                tetromino.x -= 1;
            }
        }
    }

    pub fn move_down(&mut self) {
        if self.state != GameState::Playing {
            return;
        }

        if let Some(ref mut tetromino) = self.current_tetromino {
            tetromino.y += 1;
            if !self.board.is_valid_position(tetromino) {
                tetromino.y -= 1;
                self.place_tetromino();
            }
        } else {
            self.spawn_tetromino();
        }
    }

    pub fn rotate(&mut self) {
        if self.state != GameState::Playing {
            return;
        }

        if let Some(ref mut tetromino) = self.current_tetromino {
            tetromino.rotate();
            if !self.board.is_valid_position(tetromino) {
                // Try wall kicks
                let original_x = tetromino.x;
                
                // Try moving left
                tetromino.x -= 1;
                if !self.board.is_valid_position(tetromino) {
                    tetromino.x = original_x + 1;
                    if !self.board.is_valid_position(tetromino) {
                        tetromino.x = original_x;
                        tetromino.rotate();
                        tetromino.rotate();
                        tetromino.rotate(); // Rotate back 3 times to get original rotation
                    }
                }
            }
        }
    }

    pub fn hard_drop(&mut self) {
        if self.state != GameState::Playing {
            return;
        }

        if let Some(ref mut tetromino) = self.current_tetromino {
            let mut drop_distance = 0;
            while self.board.is_valid_position(tetromino) {
                tetromino.y += 1;
                drop_distance += 1;
            }
            tetromino.y -= 1;
            
            // Add score for hard drop
            self.score += drop_distance * 2;
            
            self.place_tetromino();
        }
    }

    fn place_tetromino(&mut self) {
        if let Some(tetromino) = self.current_tetromino.take() {
            self.board.place_tetromino(&tetromino);
            
            // Clear lines and update score
            let cleared_lines = self.board.clear_lines_with_animation();
            if !cleared_lines.is_empty() {
                self.lines_cleared += cleared_lines.len() as u32;
                self.update_score(cleared_lines.len());
                self.update_level();
                
                // Add line clear animation
                self.animations.add_line_clear_animation(
                    &cleared_lines,
                    crate::board::BOARD_WIDTH,
                    self.block_size,
                    self.board.offset_x,
                    self.board.offset_y,
                );
            }
            
            // Spawn next tetromino
            self.spawn_tetromino();
        }
    }

    fn update_score(&mut self, lines_cleared: usize) {
        let points = match lines_cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        };
        
        self.score += points * self.level;
    }

    fn update_level(&mut self) {
        let new_level = (self.lines_cleared / 10) + 1;
        if new_level != self.level {
            self.level = new_level;
            self.drop_interval = (1.0 - (self.level - 1) as f32 * 0.1).max(0.1);
        }
    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            GameState::Playing => self.state = GameState::Paused,
            GameState::Paused => self.state = GameState::Playing,
            GameState::GameOver => {}
            GameState::Menu => {}
        }
    }

    pub fn reset(&mut self) {
        self.board.clear();
        self.current_tetromino = None;
        self.next_tetromino = Tetromino::random();
        self.score = 0;
        self.level = 1;
        self.lines_cleared = 0;
        self.state = GameState::Playing;
        self.drop_timer = 0.0;
        self.drop_interval = 1.0;
    }

    pub fn start_game(&mut self) {
        self.state = GameState::Playing;
        self.spawn_tetromino();
    }

    pub fn return_to_menu(&mut self) {
        self.state = GameState::Menu;
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }

    pub fn get_lines_cleared(&self) -> u32 {
        self.lines_cleared
    }

    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    pub fn get_next_tetromino(&self) -> &Tetromino {
        &self.next_tetromino
    }

    pub fn get_animations(&self) -> &AnimationManager {
        &self.animations
    }

    pub fn get_animations_mut(&mut self) -> &mut AnimationManager {
        &mut self.animations
    }
}
