mod game;
mod tetromino;
mod board;
mod audio;
mod ui;
mod config;
mod menu;
mod animations;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::input::keyboard::{KeyCode, KeyInput};

use game::Game;
use ui::UI;
use menu::Menu;
use config::{WINDOW_WIDTH, WINDOW_HEIGHT, BACKGROUND_COLOR};

struct TetrisGame {
    game: Game,
    ui: UI,
    menu: Menu,
}

impl TetrisGame {
    fn new(ctx: &mut Context) -> GameResult<TetrisGame> {
        let game = Game::new();
        let ui = UI::new(ctx)?;
        let menu = Menu::new();
        
        Ok(TetrisGame { game, ui, menu })
    }
}

impl EventHandler for TetrisGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.game.get_state() {
            game::GameState::Menu => {
                self.menu.update(ctx);
            }
            _ => {
                self.game.update(ctx);
                self.game.get_animations_mut().update(ctx.time.delta().as_secs_f32());
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let background_color = Color::new(
            BACKGROUND_COLOR.0,
            BACKGROUND_COLOR.1,
            BACKGROUND_COLOR.2,
            BACKGROUND_COLOR.3,
        );
        let mut canvas = graphics::Canvas::from_frame(ctx, background_color);
        
        match self.game.get_state() {
            game::GameState::Menu => {
                self.menu.draw(ctx, &mut canvas)?;
            }
            _ => {
                self.game.draw(ctx, &mut canvas)?;
                self.game.get_animations().draw(ctx, &mut canvas)?;
                self.ui.draw(ctx, &mut canvas, &self.game)?;
            }
        }
        
        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeat: bool,
    ) -> GameResult {
        match self.game.get_state() {
            game::GameState::Menu => {
                match input.keycode {
                    Some(KeyCode::Up) | Some(KeyCode::Down) => {
                        self.menu.select_next();
                    }
                    Some(KeyCode::Return) => {
                        match self.menu.get_selected_option() {
                            menu::MenuOption::Start => {
                                self.game.start_game();
                            }
                            menu::MenuOption::Quit => {
                                std::process::exit(0);
                            }
                        }
                    }
                    Some(KeyCode::Escape) => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            _ => {
                match input.keycode {
                    Some(KeyCode::Left) => self.game.move_left(),
                    Some(KeyCode::Right) => self.game.move_right(),
                    Some(KeyCode::Down) => self.game.move_down(),
                    Some(KeyCode::Up) => self.game.rotate(),
                    Some(KeyCode::Space) => self.game.hard_drop(),
                    Some(KeyCode::P) => self.game.toggle_pause(),
                    Some(KeyCode::R) => self.game.reset(),
                    Some(KeyCode::Escape) => {
                        self.game.return_to_menu();
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("Purple Box Destruction", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Purple Box Destruction"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;

    let game = TetrisGame::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}
