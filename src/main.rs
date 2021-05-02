extern crate ggez;

const SNAKE_INIT_POS: (i16, i16) = (5, 5);
const FRUIT_INIT_POS: (i16, i16) = (10, 10);

const PIXEL_SIZE: (i16, i16) = (20, 20);
const SIZE_IN_PIXELS: (i16, i16) = (20, 20);
const SCREEN_SIZE: (f32, f32) = (
    (PIXEL_SIZE.0 * SIZE_IN_PIXELS.0) as f32,
    (PIXEL_SIZE.1 * SIZE_IN_PIXELS.1) as f32,
);

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

use ggez::{event, graphics, conf, Context, ContextBuilder, GameResult};

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
struct Position {
    x: i16,
    y: i16,
}

impl Position {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

struct Fruit {
    pos: Position
}

impl Fruit {
    pub fn new(x: i16, y: i16) -> Self {
        Self {
            pos: Position::new(x, y),
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rect = graphics::Rect::new_i32(
            (self.pos.x).into(),
            (self.pos.y).into(),
            (PIXEL_SIZE.0 - 1).into(),
            (PIXEL_SIZE.1 - 1).into()
        );
        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, RED.into())?;
        graphics::draw(ctx, &mesh, ggez::graphics::DrawParam::default())        
    }
}

struct Snake {
    head: Position,
    direction: Direction,
}

impl Snake {
    pub fn new(x: i16, y: i16) -> Self {
        let direction = Direction::Right;

        Self {
            direction,
            head: Position::new(x, y),
        }
    }

    fn update(&mut self, fruit: &Fruit) -> GameResult<()> {
        Ok(())
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rect = graphics::Rect::new_i32(
            (self.head.x).into(),
            (self.head.y).into(),
            (PIXEL_SIZE.0 - 1).into(),
            (PIXEL_SIZE.1 - 1).into()
        );
        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, GREEN.into())?;
        graphics::draw(ctx, &mesh, ggez::graphics::DrawParam::default())        
    }
}

struct Game {
    snake: Snake,
    fruit: Fruit,
}

impl Game {
    fn new() -> Self {
        Self {
            snake: Snake::new(SNAKE_INIT_POS.0, SNAKE_INIT_POS.1),
            fruit: Fruit::new(FRUIT_INIT_POS.0, FRUIT_INIT_POS.1),
        }
    }
}

impl event::EventHandler for Game {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        self.fruit.draw(ctx)?;
        self.snake.draw(ctx)?;

        graphics::present(ctx)
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.snake.update(&self.fruit)?;
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