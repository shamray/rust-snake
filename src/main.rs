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
const DEFAULT_ACCEL: i16 = 1;

use ggez::{event, graphics, conf, Context, ContextBuilder, GameResult};
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Copy, Clone)]
struct Position {
    x: i16,
    y: i16,
}

impl Position {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn new_by_direction(x: i16, y: i16, direction: Direction) -> Self {
        let accel = DEFAULT_ACCEL;

        let (x, y) = match direction {
            Direction::Up => (x, y - accel),
            Direction::Down => (x, y + accel),
            Direction::Left => (x - accel, y),
            Direction::Right => (x + accel, y),
        };

        Self { x, y }
    }
}

impl From<Position> for graphics::Rect {
    fn from(pos: Position) -> Self {
        graphics::Rect::new_i32(
            (pos.x * PIXEL_SIZE.0).into(),
            (pos.y * PIXEL_SIZE.0).into(),
            (PIXEL_SIZE.0 - 1).into(),
            (PIXEL_SIZE.1 - 1).into(),
        )
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
        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.pos.into(), RED.into())?;
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
        let new_head = Position::new_by_direction(self.head.x, self.head.y, self.direction);
        self.head = new_head;

        Ok(())
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.head.into(), GREEN.into())?;
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

    fn gameover(_ctx: &Context) {

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

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool
    ) {
        if keycode == ggez::input::keyboard::KeyCode::Escape {
            Game::gameover(ctx);
        }
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