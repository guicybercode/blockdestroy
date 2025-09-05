use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect, Text, TextFragment};
use ggez::mint::Point2;

use crate::game::{Game, GameState};
use crate::tetromino::Tetromino;
use crate::config::{MENU_TEXT_COLOR, MENU_HIGHLIGHT_COLOR, BORDER_COLOR};

pub struct UI {}

impl UI {
    pub fn new(_ctx: &mut Context) -> GameResult<UI> {
        Ok(UI {})
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas, game: &Game) -> GameResult {
        let text_color = Color::new(MENU_TEXT_COLOR.0, MENU_TEXT_COLOR.1, MENU_TEXT_COLOR.2, MENU_TEXT_COLOR.3);
        
        // Draw score
        self.draw_text(canvas, &format!("Score: {}", game.get_score()), 550.0, 50.0, 24.0, text_color)?;
        
        // Draw level
        self.draw_text(canvas, &format!("Level: {}", game.get_level()), 550.0, 80.0, 24.0, text_color)?;
        
        // Draw lines cleared
        self.draw_text(canvas, &format!("Lines: {}", game.get_lines_cleared()), 550.0, 110.0, 24.0, text_color)?;
        
        // Draw next piece preview
        self.draw_text(canvas, "Next:", 550.0, 150.0, 20.0, text_color)?;
        self.draw_next_piece(ctx, canvas, game.get_next_tetromino(), 550.0, 180.0)?;
        
        // Draw controls
        self.draw_controls(canvas)?;
        
        // Draw game state messages
        let highlight_color = Color::new(MENU_HIGHLIGHT_COLOR.0, MENU_HIGHLIGHT_COLOR.1, MENU_HIGHLIGHT_COLOR.2, MENU_HIGHLIGHT_COLOR.3);
        match game.get_state() {
            GameState::Paused => {
                self.draw_text(canvas, "PAUSED", 550.0, 300.0, 32.0, highlight_color)?;
                self.draw_text(canvas, "Press P to resume", 550.0, 340.0, 16.0, text_color)?;
            }
            GameState::GameOver => {
                self.draw_text(canvas, "GAME OVER", 550.0, 300.0, 32.0, highlight_color)?;
                self.draw_text(canvas, "Press R to restart", 550.0, 340.0, 16.0, text_color)?;
            }
            GameState::Playing => {}
            GameState::Menu => {}
        }
        
        Ok(())
    }

    fn draw_text(&self, canvas: &mut graphics::Canvas, text: &str, x: f32, y: f32, size: f32, color: Color) -> GameResult {
        let text_fragment = TextFragment::new(text)
            .scale(size)
            .color(color);
        
        let text_obj = Text::new(text_fragment);
        let dest = Point2 { x, y };
        
        canvas.draw(&text_obj, DrawParam::default().dest(dest));
        Ok(())
    }

    fn draw_next_piece(&self, ctx: &mut Context, canvas: &mut graphics::Canvas, tetromino: &Tetromino, x: f32, y: f32) -> GameResult {
        let block_size = 20.0;
        let offset_x = x + 20.0;
        let offset_y = y + 20.0;
        
        // Draw background for next piece preview
        let preview_rect = Rect::new(x, y, 100.0, 80.0);
        let border_color = Color::new(BORDER_COLOR.0, BORDER_COLOR.1, BORDER_COLOR.2, BORDER_COLOR.3);
        let preview_mesh = Mesh::new_rectangle(ctx, DrawMode::stroke(2.0), preview_rect, border_color)?;
        canvas.draw(&preview_mesh, DrawParam::default());
        
        // Draw the next tetromino
        tetromino.draw(ctx, canvas, block_size, offset_x, offset_y)?;
        
        Ok(())
    }

    fn draw_controls(&self, canvas: &mut graphics::Canvas) -> GameResult {
        let controls = vec![
            "Controls:",
            "Arrow Keys - Move",
            "Up - Rotate",
            "Space - Hard Drop",
            "P - Pause",
            "R - Restart",
            "ESC - Menu",
        ];
        
        let text_color = Color::new(MENU_TEXT_COLOR.0, MENU_TEXT_COLOR.1, MENU_TEXT_COLOR.2, MENU_TEXT_COLOR.3);
        let highlight_color = Color::new(MENU_HIGHLIGHT_COLOR.0, MENU_HIGHLIGHT_COLOR.1, MENU_HIGHLIGHT_COLOR.2, MENU_HIGHLIGHT_COLOR.3);
        
        for (i, control) in controls.iter().enumerate() {
            let y = 450.0 + (i as f32 * 20.0);
            let color = if i == 0 { highlight_color } else { text_color };
            let size = if i == 0 { 18.0 } else { 14.0 };
            
            self.draw_text(canvas, control, 550.0, y, size, color)?;
        }
        
        Ok(())
    }
}
