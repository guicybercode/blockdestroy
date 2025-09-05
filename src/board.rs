use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::{Context, GameResult};
use crate::tetromino::Tetromino;
use crate::config::{GRID_COLOR, BORDER_COLOR, GHOST_COLOR};

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 15;

#[derive(Debug, Clone)]
pub struct Board {
    pub grid: Vec<Vec<Option<Color>>>,
    pub block_size: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl Board {
    pub fn new(block_size: f32, offset_x: f32, offset_y: f32) -> Self {
        let grid = vec![vec![None; BOARD_WIDTH]; BOARD_HEIGHT];
        Board {
            grid,
            block_size,
            offset_x,
            offset_y,
        }
    }

    pub fn clear(&mut self) {
        self.grid = vec![vec![None; BOARD_WIDTH]; BOARD_HEIGHT];
    }

    pub fn is_valid_position(&self, tetromino: &Tetromino) -> bool {
        for (y, row) in tetromino.blocks.iter().enumerate() {
            for (x, &block) in row.iter().enumerate() {
                if block {
                    let board_x = tetromino.x + x as i32;
                    let board_y = tetromino.y + y as i32;
                    
                    // Check boundaries
                    if board_x < 0 || board_x >= BOARD_WIDTH as i32 || 
                       board_y < 0 || board_y >= BOARD_HEIGHT as i32 {
                        return false;
                    }
                    
                    // Check collision with existing blocks
                    if self.grid[board_y as usize][board_x as usize].is_some() {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn place_tetromino(&mut self, tetromino: &Tetromino) {
        let color = tetromino.get_color();
        for (y, row) in tetromino.blocks.iter().enumerate() {
            for (x, &block) in row.iter().enumerate() {
                if block {
                    let board_x = (tetromino.x + x as i32) as usize;
                    let board_y = (tetromino.y + y as i32) as usize;
                    if board_y < BOARD_HEIGHT && board_x < BOARD_WIDTH {
                        self.grid[board_y][board_x] = Some(color);
                    }
                }
            }
        }
    }

    pub fn clear_lines(&mut self) -> usize {
        let mut lines_cleared = 0;
        let mut y = BOARD_HEIGHT - 1;
        
        while y > 0 {
            if self.is_line_full(y) {
                self.remove_line(y);
                lines_cleared += 1;
            } else {
                y -= 1;
            }
        }
        
        lines_cleared
    }

    pub fn clear_lines_with_animation(&mut self) -> Vec<usize> {
        let mut cleared_lines = Vec::new();
        let mut y = BOARD_HEIGHT - 1;
        
        while y > 0 {
            if self.is_line_full(y) {
                cleared_lines.push(y);
                self.remove_line(y);
            } else {
                y -= 1;
            }
        }
        
        cleared_lines
    }

    fn is_line_full(&self, y: usize) -> bool {
        self.grid[y].iter().all(|cell| cell.is_some())
    }

    fn remove_line(&mut self, y: usize) {
        self.grid.remove(y);
        self.grid.insert(0, vec![None; BOARD_WIDTH]);
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        // Draw background grid
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let rect = Rect::new(
                    x as f32 * self.block_size + self.offset_x,
                    y as f32 * self.block_size + self.offset_y,
                    self.block_size,
                    self.block_size,
                );
                
                // Draw grid lines
                let grid_color = Color::new(GRID_COLOR.0, GRID_COLOR.1, GRID_COLOR.2, GRID_COLOR.3);
                let grid_mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(1.0), rect, grid_color)?;
                canvas.draw(&grid_mesh, DrawParam::default());
                
                // Draw placed blocks
                if let Some(color) = self.grid[y][x] {
                    let block_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color)?;
                    canvas.draw(&block_mesh, DrawParam::default());
                    
                    // Draw border for placed blocks
                    let border_color = Color::new(BORDER_COLOR.0, BORDER_COLOR.1, BORDER_COLOR.2, BORDER_COLOR.3);
                    let border_mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(2.0), rect, border_color)?;
                    canvas.draw(&border_mesh, DrawParam::default());
                }
            }
        }
        
        // Draw board border
        let border_rect = Rect::new(
            self.offset_x - 2.0,
            self.offset_y - 2.0,
            BOARD_WIDTH as f32 * self.block_size + 4.0,
            BOARD_HEIGHT as f32 * self.block_size + 4.0,
        );
        let border_color = Color::new(BORDER_COLOR.0, BORDER_COLOR.1, BORDER_COLOR.2, BORDER_COLOR.3);
        let border_mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(4.0), border_rect, border_color)?;
        canvas.draw(&border_mesh, DrawParam::default());
        
        Ok(())
    }

    pub fn get_ghost_position(&self, tetromino: &Tetromino) -> (i32, i32) {
        let mut ghost_y = tetromino.y;
        
        while ghost_y < BOARD_HEIGHT as i32 {
            let mut test_tetromino = tetromino.clone();
            test_tetromino.y = ghost_y + 1;
            
            if !self.is_valid_position(&test_tetromino) {
                break;
            }
            ghost_y += 1;
        }
        
        (tetromino.x, ghost_y)
    }

    pub fn draw_ghost(&self, ctx: &mut Context, canvas: &mut graphics::Canvas, tetromino: &Tetromino) -> GameResult {
        let (ghost_x, ghost_y) = self.get_ghost_position(tetromino);
        
        for (y, row) in tetromino.blocks.iter().enumerate() {
            for (x, &block) in row.iter().enumerate() {
                if block {
                    let rect = Rect::new(
                        (ghost_x + x as i32) as f32 * self.block_size + self.offset_x,
                        (ghost_y + y as i32) as f32 * self.block_size + self.offset_y,
                        self.block_size,
                        self.block_size,
                    );
                    
                    let ghost_color = Color::new(GHOST_COLOR.0, GHOST_COLOR.1, GHOST_COLOR.2, GHOST_COLOR.3);
                    let mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(2.0), rect, ghost_color)?;
                    canvas.draw(&mesh, DrawParam::default());
                }
            }
        }
        
        Ok(())
    }
}
