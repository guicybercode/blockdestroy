use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect};
use std::collections::VecDeque;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub color: Color,
    pub life: f32,
    pub max_life: f32,
    pub size: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(50.0..150.0);
        
        Particle {
            x,
            y,
            velocity_x: angle.cos() * speed,
            velocity_y: angle.sin() * speed,
            color,
            life: 1.0,
            max_life: 1.0,
            size: rng.gen_range(3.0..8.0),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.x += self.velocity_x * dt;
        self.y += self.velocity_y * dt;
        self.velocity_y += 200.0 * dt; // Gravity
        self.life -= dt * 2.0; // Fade out over 0.5 seconds
    }

    pub fn is_alive(&self) -> bool {
        self.life > 0.0
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        if self.life <= 0.0 {
            return Ok(());
        }

        let alpha = self.life;
        let mut color = self.color;
        color.a = alpha;

        let rect = Rect::new(
            self.x - self.size / 2.0,
            self.y - self.size / 2.0,
            self.size,
            self.size,
        );

        let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, color)?;
        canvas.draw(&mesh, DrawParam::default());

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LineClearAnimation {
    pub particles: Vec<Particle>,
    pub duration: f32,
    pub elapsed: f32,
}

impl LineClearAnimation {
    pub fn new(cleared_lines: &[usize], board_width: usize, block_size: f32, offset_x: f32, offset_y: f32) -> Self {
        let mut particles = Vec::new();
        
        for &line_y in cleared_lines {
            for x in 0..board_width {
                let particle_x = x as f32 * block_size + offset_x + block_size / 2.0;
                let particle_y = line_y as f32 * block_size + offset_y + block_size / 2.0;
                
                // Create multiple particles per block for more dramatic effect
                for _ in 0..3 {
                    let color = Color::new(
                        0.8 + rand::thread_rng().gen_range(-0.2..0.2),
                        0.4 + rand::thread_rng().gen_range(-0.2..0.2),
                        1.0,
                        1.0,
                    );
                    particles.push(Particle::new(particle_x, particle_y, color));
                }
            }
        }

        LineClearAnimation {
            particles,
            duration: 1.0, // Animation lasts 1 second
            elapsed: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
        self.particles.retain_mut(|particle| {
            particle.update(dt);
            particle.is_alive()
        });
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration || self.particles.is_empty()
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        for particle in &self.particles {
            particle.draw(ctx, canvas)?;
        }
        Ok(())
    }
}

pub struct AnimationManager {
    pub line_clear_animations: VecDeque<LineClearAnimation>,
}

impl AnimationManager {
    pub fn new() -> Self {
        AnimationManager {
            line_clear_animations: VecDeque::new(),
        }
    }

    pub fn add_line_clear_animation(&mut self, cleared_lines: &[usize], board_width: usize, block_size: f32, offset_x: f32, offset_y: f32) {
        let animation = LineClearAnimation::new(cleared_lines, board_width, block_size, offset_x, offset_y);
        self.line_clear_animations.push_back(animation);
    }

    pub fn update(&mut self, dt: f32) {
        // Update all animations
        for animation in &mut self.line_clear_animations {
            animation.update(dt);
        }

        // Remove finished animations
        self.line_clear_animations.retain(|animation| !animation.is_finished());
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        for animation in &self.line_clear_animations {
            animation.draw(ctx, canvas)?;
        }
        Ok(())
    }

    pub fn is_animating(&self) -> bool {
        !self.line_clear_animations.is_empty()
    }
}
