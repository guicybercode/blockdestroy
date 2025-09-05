use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::{Context, GameResult};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
    U,
    V,
    W,
    X,
    Y,
    ZCustom,
}

#[derive(Debug, Clone)]
pub struct Tetromino {
    pub tetromino_type: TetrominoType,
    pub rotation: usize,
    pub x: i32,
    pub y: i32,
    pub blocks: Vec<Vec<bool>>,
}

impl Tetromino {
    pub fn new(tetromino_type: TetrominoType) -> Self {
        let blocks = Self::get_blocks(tetromino_type, 0);
        Tetromino {
            tetromino_type,
            rotation: 0,
            x: 6,
            y: 0,
            blocks,
        }
    }

    pub fn random() -> Self {
        let types = [
            TetrominoType::I,
            TetrominoType::O,
            TetrominoType::T,
            TetrominoType::S,
            TetrominoType::Z,
            TetrominoType::J,
            TetrominoType::L,
            TetrominoType::U,
            TetrominoType::V,
            TetrominoType::W,
            TetrominoType::X,
            TetrominoType::Y,
            TetrominoType::ZCustom,
        ];
        let mut rng = rand::thread_rng();
        let tetromino_type = types[rng.gen_range(0..types.len())];
        Self::new(tetromino_type)
    }

    pub fn rotate(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
        self.blocks = Self::get_blocks(self.tetromino_type, self.rotation);
    }

    pub fn get_color(&self) -> Color {
        match self.tetromino_type {
            TetrominoType::I => Color::new(0.8, 0.4, 1.0, 1.0), // Bright purple
            TetrominoType::O => Color::new(0.7, 0.3, 0.9, 1.0), // Medium purple
            TetrominoType::T => Color::new(0.6, 0.2, 0.8, 1.0), // Dark purple
            TetrominoType::S => Color::new(0.5, 0.1, 0.7, 1.0), // Darker purple
            TetrominoType::Z => Color::new(0.4, 0.0, 0.6, 1.0), // Very dark purple
            TetrominoType::J => Color::new(0.9, 0.5, 1.0, 1.0), // Light magenta
            TetrominoType::L => Color::new(0.3, 0.0, 0.5, 1.0), // Deep purple
            TetrominoType::U => Color::new(0.8, 0.6, 1.0, 1.0), // Light purple
            TetrominoType::V => Color::new(0.6, 0.4, 0.8, 1.0), // Medium-light purple
            TetrominoType::W => Color::new(0.4, 0.2, 0.6, 1.0), // Medium-dark purple
            TetrominoType::X => Color::new(0.9, 0.7, 1.0, 1.0), // Very light purple
            TetrominoType::Y => Color::new(0.7, 0.5, 0.9, 1.0), // Light-medium purple
            TetrominoType::ZCustom => Color::new(0.5, 0.3, 0.7, 1.0), // Custom purple
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas, block_size: f32, offset_x: f32, offset_y: f32) -> GameResult {
        let color = self.get_color();
        
        for (y, row) in self.blocks.iter().enumerate() {
            for (x, &block) in row.iter().enumerate() {
                if block {
                    let rect = Rect::new(
                        (self.x + x as i32) as f32 * block_size + offset_x,
                        (self.y + y as i32) as f32 * block_size + offset_y,
                        block_size,
                        block_size,
                    );
                    
                    let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color)?;
                    canvas.draw(&mesh, DrawParam::default());
                    
                    // Draw border
                    let border_color = Color::new(0.9, 0.5, 1.0, 1.0); // Light purple border
                    let border_mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(2.0), rect, border_color)?;
                    canvas.draw(&border_mesh, DrawParam::default());
                }
            }
        }
        Ok(())
    }

    fn get_blocks(tetromino_type: TetrominoType, rotation: usize) -> Vec<Vec<bool>> {
        let blocks = match tetromino_type {
            TetrominoType::I => vec![
                vec![
                    vec![false, false, false, false],
                    vec![true, true, true, true],
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, false, true, false],
                    vec![false, false, true, false],
                    vec![false, false, true, false],
                    vec![false, false, true, false],
                ],
                vec![
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                    vec![true, true, true, true],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, true, false, false],
                    vec![false, true, false, false],
                    vec![false, true, false, false],
                    vec![false, true, false, false],
                ],
            ],
            TetrominoType::O => vec![
                vec![
                    vec![false, true, true, false],
                    vec![false, true, true, false],
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                ],
            ],
            TetrominoType::T => vec![
                vec![
                    vec![false, true, false, false],
                    vec![true, true, true, false],
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, true, false, false],
                    vec![false, true, true, false],
                    vec![false, true, false, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, false, false, false],
                    vec![true, true, true, false],
                    vec![false, true, false, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, true, false, false],
                    vec![true, true, false, false],
                    vec![false, true, false, false],
                    vec![false, false, false, false],
                ],
            ],
            TetrominoType::S => vec![
                vec![
                    vec![false, true, true, false],
                    vec![true, true, false, false],
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, true, false, false],
                    vec![false, true, true, false],
                    vec![false, false, true, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, false, false, false],
                    vec![false, true, true, false],
                    vec![true, true, false, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![true, false, false, false],
                    vec![true, true, false, false],
                    vec![false, true, false, false],
                    vec![false, false, false, false],
                ],
            ],
            TetrominoType::Z => vec![
                vec![
                    vec![true, true, false, false],
                    vec![false, true, true, false],
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, false, true, false],
                    vec![false, true, true, false],
                    vec![false, true, false, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, false, false, false],
                    vec![true, true, false, false],
                    vec![false, true, true, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, true, false, false],
                    vec![true, true, false, false],
                    vec![true, false, false, false],
                    vec![false, false, false, false],
                ],
            ],
            TetrominoType::J => vec![
                vec![
                    vec![true, false, false, false],
                    vec![true, true, true, false],
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, true, true, false],
                    vec![false, true, false, false],
                    vec![false, true, false, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, false, false, false],
                    vec![true, true, true, false],
                    vec![false, false, true, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, true, false, false],
                    vec![false, true, false, false],
                    vec![true, true, false, false],
                    vec![false, false, false, false],
                ],
            ],
            TetrominoType::L => vec![
                vec![
                    vec![false, false, true, false],
                    vec![true, true, true, false],
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, true, false, false],
                    vec![false, true, false, false],
                    vec![false, true, true, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, false, false, false],
                    vec![true, true, true, false],
                    vec![true, false, false, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![true, true, false, false],
                    vec![false, true, false, false],
                    vec![false, true, false, false],
                    vec![false, false, false, false],
                ],
            ],
            TetrominoType::U => vec![
                vec![
                    vec![true, false, true, false],
                    vec![true, true, true, false],
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, true, true, false],
                    vec![false, true, false, false],
                    vec![false, true, true, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, false, false, false],
                    vec![true, true, true, false],
                    vec![true, false, true, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![true, true, false, false],
                    vec![false, true, false, false],
                    vec![true, true, false, false],
                    vec![false, false, false, false],
                ],
            ],
            TetrominoType::V => vec![
                vec![
                    vec![true, false, false, false],
                    vec![true, false, false, false],
                    vec![true, true, true, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, false, true, false],
                    vec![false, false, true, false],
                    vec![true, true, true, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, false, false, false],
                    vec![true, true, true, false],
                    vec![false, false, true, false],
                    vec![false, false, true, false],
                ],
                vec![
                    vec![true, true, true, false],
                    vec![true, false, false, false],
                    vec![true, false, false, false],
                    vec![false, false, false, false],
                ],
            ],
            TetrominoType::W => vec![
                vec![
                    vec![true, false, false, false],
                    vec![true, true, false, false],
                    vec![false, true, true, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, false, true, false],
                    vec![false, true, true, false],
                    vec![false, true, false, false],
                    vec![false, true, false, false],
                ],
                vec![
                    vec![false, false, false, false],
                    vec![true, true, false, false],
                    vec![false, true, true, false],
                    vec![false, false, true, false],
                ],
                vec![
                    vec![false, true, false, false],
                    vec![true, true, false, false],
                    vec![true, false, false, false],
                    vec![false, false, false, false],
                ],
            ],
            TetrominoType::X => vec![
                vec![
                    vec![false, true, false, false],
                    vec![true, true, true, false],
                    vec![false, true, false, false],
                    vec![false, false, false, false],
                ],
            ],
            TetrominoType::Y => vec![
                vec![
                    vec![false, true, false, false],
                    vec![true, true, true, false],
                    vec![false, true, false, false],
                    vec![false, true, false, false],
                ],
                vec![
                    vec![false, true, true, false],
                    vec![false, true, false, false],
                    vec![false, true, true, false],
                    vec![false, true, false, false],
                ],
                vec![
                    vec![false, true, false, false],
                    vec![false, true, false, false],
                    vec![true, true, true, false],
                    vec![false, true, false, false],
                ],
                vec![
                    vec![false, true, false, false],
                    vec![true, true, false, false],
                    vec![false, true, false, false],
                    vec![true, true, false, false],
                ],
            ],
            TetrominoType::ZCustom => vec![
                vec![
                    vec![true, true, false, false],
                    vec![false, true, true, false],
                    vec![false, false, true, false],
                    vec![false, false, false, false],
                ],
                vec![
                    vec![false, false, true, false],
                    vec![false, true, true, false],
                    vec![false, true, false, false],
                    vec![false, true, false, false],
                ],
                vec![
                    vec![false, false, false, false],
                    vec![true, false, false, false],
                    vec![true, true, false, false],
                    vec![false, true, true, false],
                ],
                vec![
                    vec![false, true, false, false],
                    vec![false, true, false, false],
                    vec![false, true, true, false],
                    vec![true, false, false, false],
                ],
            ],
        };
        
        blocks[rotation % blocks.len()].clone()
    }
}
