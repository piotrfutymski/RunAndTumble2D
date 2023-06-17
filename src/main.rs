pub mod system;
use ggez::graphics::Rect;
use system::System;
use system::VertexState;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let window_size = 800.0;

    if args.len() < 6 {
        panic!("not enough arguments");
    }

    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(ggez::conf::WindowMode::default().dimensions(window_size, window_size))
        .build()
        .expect("aieee, could not create ggez context!");
    let my_game = Simulation::new(&mut ctx, window_size, &args);
    event::run(ctx, event_loop, my_game);
}


struct Simulation {
    system: System,
    rect_size: f32,
    distance: u32,
    file: File,
    step_num: u32,
    max_steps: u32
}

impl Simulation {
    pub fn new(_ctx: &mut Context, window_size: f32, args: &Vec<String>) -> Simulation {
        Simulation {
            system: System::new(args[5].parse::<f64>().unwrap(), args[1].parse::<usize>().unwrap(), args[2].parse::<i32>().unwrap()),
            rect_size: window_size/args[1].parse::<f32>().unwrap(),
            distance: args[3].parse::<u32>().unwrap(),
            file: File::create(format!("data_{}_{}_{}.txt", args[1], args[2], args[3])).expect("Can't create file"),
            step_num: 0,
            max_steps: args[4].parse::<u32>().unwrap()
        }
    }
}

impl EventHandler for Simulation {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.system.step();
        let r = self.system.calculate_distribution_function(self.distance);
        println!("{}", r);
        write!(self.file, "{}\n", r).unwrap();
        self.step_num+=1;
        if self.step_num == self.max_steps {
            ctx.request_quit();
        }
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