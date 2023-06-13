pub mod system;
use ggez::graphics::Rect;
use system::System;
use system::VertexState;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

fn main() {
    let window_size = 800.0;

    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(ggez::conf::WindowMode::default().dimensions(window_size, window_size))
        .build()
        .expect("aieee, could not create ggez context!");
    let my_game = Simulation::new(&mut ctx, window_size);
    event::run(ctx, event_loop, my_game);
}


struct Simulation {
    system: System,
    rect_size: f32
}

impl Simulation {
    pub fn new(_ctx: &mut Context, window_size: f32) -> Simulation {
        Simulation {
            system: System::new(0.02, 200, 800),
            rect_size: window_size/200.0,
        }
    }
}

impl EventHandler for Simulation {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.system.step();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx,  graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let particles = self.system.get_particles();
        for i in 0..particles.len() {
            for j in 0..particles.len() {
                if particles[i][j] != VertexState::None {
                    let rectangle = Rect::new(i as f32*self.rect_size, j as f32*self.rect_size, self.rect_size, self.rect_size);
                    let rectangle_color = Color::new(0.0, 0.0, 1.0, 1.0);
                    canvas.draw(&graphics::Quad, graphics::DrawParam::new()
                    .dest_rect(rectangle)
                    .color(rectangle_color))
                }
            }
        }
        canvas.finish(ctx)
    }
}