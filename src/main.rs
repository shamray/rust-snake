extern crate ggez;

const PIXEL_SIZE: (i16, i16) = (20, 20);
const SIZE_IN_PIXELS: (i16, i16) = (20, 20);
const SCREEN_SIZE: (f32, f32) = (
    (PIXEL_SIZE.0 * SIZE_IN_PIXELS.0) as f32,
    (PIXEL_SIZE.1 * SIZE_IN_PIXELS.1) as f32,
);

use ggez::{event, graphics, conf, Context, ContextBuilder, GameResult};

struct Game {
}

impl Game {
    fn new() -> Self {
        Self {}
    }
}

impl event::EventHandler for Game {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        graphics::present(ctx)
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}

pub fn main() -> GameResult { 
    let (ctx, event_loop) = &mut ContextBuilder::new("snakegame", "shamray")
        .window_setup(conf::WindowSetup::default().title("Snake! SNAAAKE!"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("Failed to build ggez context builder");

    let game = &mut Game::new();

    event::run(ctx, event_loop, game)
}