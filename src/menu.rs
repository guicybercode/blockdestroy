use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect, Text, TextFragment};
use ggez::mint::Point2;
use crate::config::*;

#[derive(Debug, Clone, PartialEq)]
pub enum MenuOption {
    Start,
    Quit,
}

pub struct Menu {
    selected_option: MenuOption,
    animation_timer: f32,
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            selected_option: MenuOption::Start,
            animation_timer: 0.0,
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let dt = ctx.time.delta().as_secs_f32();
        self.animation_timer += dt;
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        // Draw background
        let background_color = Color::new(
            MENU_BACKGROUND_COLOR.0,
            MENU_BACKGROUND_COLOR.1,
            MENU_BACKGROUND_COLOR.2,
            MENU_BACKGROUND_COLOR.3,
        );
        
        let background_rect = Rect::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
        let background_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), background_rect, background_color)?;
        canvas.draw(&background_mesh, DrawParam::default());

        // Draw title
        self.draw_title(ctx, canvas)?;

        // Draw menu options
        self.draw_menu_options(ctx, canvas)?;

        // Draw controls
        self.draw_controls(ctx, canvas)?;

        Ok(())
    }

    fn draw_title(&self, _ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let title_color = Color::new(
            MENU_HIGHLIGHT_COLOR.0,
            MENU_HIGHLIGHT_COLOR.1,
            MENU_HIGHLIGHT_COLOR.2,
            MENU_HIGHLIGHT_COLOR.3,
        );

        // Animated title
        let pulse = (self.animation_timer * 2.0).sin() * 0.1 + 0.9;
        let animated_color = Color::new(
            title_color.r * pulse,
            title_color.g * pulse,
            title_color.b * pulse,
            title_color.a,
        );

        self.draw_text(canvas, "PURPLE BOX DESTRUCTION", WINDOW_WIDTH / 2.0 - 200.0, 100.0, 36.0, animated_color)?;

        Ok(())
    }

    fn draw_menu_options(&self, _ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let center_x = WINDOW_WIDTH / 2.0;
        let start_y = 250.0;

        // Start option
        let start_color = if self.selected_option == MenuOption::Start {
            Color::new(
                MENU_HIGHLIGHT_COLOR.0,
                MENU_HIGHLIGHT_COLOR.1,
                MENU_HIGHLIGHT_COLOR.2,
                MENU_HIGHLIGHT_COLOR.3,
            )
        } else {
            Color::new(
                MENU_TEXT_COLOR.0,
                MENU_TEXT_COLOR.1,
                MENU_TEXT_COLOR.2,
                MENU_TEXT_COLOR.3,
            )
        };

        let start_text = if self.selected_option == MenuOption::Start {
            "> START GAME <"
        } else {
            "  START GAME  "
        };

        self.draw_text(canvas, start_text, center_x - 100.0, start_y, 32.0, start_color)?;

        // Quit option
        let quit_color = if self.selected_option == MenuOption::Quit {
            Color::new(
                MENU_HIGHLIGHT_COLOR.0,
                MENU_HIGHLIGHT_COLOR.1,
                MENU_HIGHLIGHT_COLOR.2,
                MENU_HIGHLIGHT_COLOR.3,
            )
        } else {
            Color::new(
                MENU_TEXT_COLOR.0,
                MENU_TEXT_COLOR.1,
                MENU_TEXT_COLOR.2,
                MENU_TEXT_COLOR.3,
            )
        };

        let quit_text = if self.selected_option == MenuOption::Quit {
            "> QUIT GAME <"
        } else {
            "  QUIT GAME  "
        };

        self.draw_text(canvas, quit_text, center_x - 100.0, start_y + 60.0, 32.0, quit_color)?;

        Ok(())
    }

    fn draw_controls(&self, _ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let controls = vec![
            "Controls:",
            "Arrow Keys - Navigate",
            "Enter - Select",
            "ESC - Quit",
        ];

        let start_y = 450.0;
        for (i, control) in controls.iter().enumerate() {
            let y = start_y + (i as f32 * 25.0);
            let color = if i == 0 {
                Color::new(
                    MENU_HIGHLIGHT_COLOR.0,
                    MENU_HIGHLIGHT_COLOR.1,
                    MENU_HIGHLIGHT_COLOR.2,
                    MENU_HIGHLIGHT_COLOR.3,
                )
            } else {
                Color::new(
                    MENU_TEXT_COLOR.0,
                    MENU_TEXT_COLOR.1,
                    MENU_TEXT_COLOR.2,
                    MENU_TEXT_COLOR.3,
                )
            };
            let size = if i == 0 { 20.0 } else { 16.0 };

            self.draw_text(canvas, control, WINDOW_WIDTH / 2.0 - 100.0, y, size, color)?;
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

    pub fn select_next(&mut self) {
        self.selected_option = match self.selected_option {
            MenuOption::Start => MenuOption::Quit,
            MenuOption::Quit => MenuOption::Start,
        };
    }

    pub fn select_previous(&mut self) {
        self.selected_option = match self.selected_option {
            MenuOption::Start => MenuOption::Quit,
            MenuOption::Quit => MenuOption::Start,
        };
    }

    pub fn get_selected_option(&self) -> &MenuOption {
        &self.selected_option
    }
}
