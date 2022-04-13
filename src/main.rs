const SNAKE_INIT_POS: (i16, i16) = (5, 5);
const FRUIT_INIT_POS: (i16, i16) = (10, 10);

const PIXEL_SIZE: (i16, i16) = (20, 20);
const SIZE_IN_PIXELS: (i16, i16) = (20, 20);
const SCREEN_SIZE: (i32, i32) = (
    (PIXEL_SIZE.0 * SIZE_IN_PIXELS.0) as i32,
    (PIXEL_SIZE.1 * SIZE_IN_PIXELS.1) as i32,
);

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const DEFAULT_ACCEL: i16 = 1;
const DEFAULT_FPS: u16 = 60;

extern crate good_web_game as ggez;

use ggez::{event, graphics, Context, GameResult, timer};
use ggez::input::keyboard::KeyCode;
use ggez::input::keyboard::KeyMods;

use rand::Rng;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn is_opposite(d1: Direction, d2: Direction) -> bool {
    match (d1, d2) {
        (Direction::Up, Direction::Down) | 
        (Direction::Down, Direction::Up) |
        (Direction::Left, Direction::Right) | 
        (Direction::Right, Direction::Left) => true,
        _ => false,
    }
}

impl Direction {
    pub fn from_keycode(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None
        }
    }
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

        let (mut x, mut y) = match direction {
            Direction::Up => (x, y - accel),
            Direction::Down => (x, y + accel),
            Direction::Left => (x - accel, y),
            Direction::Right => (x + accel, y),
        };

        if x < 0 {
            x = PIXEL_SIZE.0;
        } else if x >= PIXEL_SIZE.0 {
            x = 0;
        }

        if y < 0 {
            y = PIXEL_SIZE.0;
        } else if y >= PIXEL_SIZE.0 {
            y = 0;
        }

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

    fn regenerate(&mut self) {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0..SIZE_IN_PIXELS.0) as i16;
        let y = rng.gen_range(0..SIZE_IN_PIXELS.1) as i16;

        self.pos = Position::new(x, y)
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.pos.into(), RED.into())?;
        graphics::draw(ctx, &mesh, ggez::graphics::DrawParam::default())        
    }
}

enum SnakeState {
    AteFruit,
    SelfCollision,
}

struct Snake {
    head: Position,
    body: Vec<Position>,
    direction: Direction,
    state: Option<SnakeState>,
}

impl Snake {
    pub fn new(x: i16, y: i16) -> Self {
        let direction = Direction::Right;
        let body = Vec::<Position>::new();
        // body.push(Position::new(x, y));
        // body.push(Position::new_by_direction(x, y, direction));

        Self {
            head: Position::new(x, y),
            state: None,
            body,
            direction,
        }
    }

    fn reverse(&mut self) {
        self.body.insert(0, self.head);
        self.body.reverse();
        self.head = self.body.pop().unwrap();
    }

    fn reset(&mut self) {
        self.body = Vec::new()
    }

    fn self_collision(&self) -> bool{
        for segment in &self.body {
            if self.head == *segment {
                return true;
            }
        }
        false
    }

    fn update(&mut self, fruit: &Fruit) -> GameResult<()> {
        let new_head = Position::new_by_direction(self.head.x, self.head.y, self.direction);
        self.body.insert(0, self.head);
        self.head = new_head;

        if self.head == fruit.pos {
            self.state = Some(SnakeState::AteFruit)
        } else if self.self_collision() {
            self.state = Some(SnakeState::SelfCollision)
        } else {
            self.body.pop();
            self.state = None;
        }

        Ok(())
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let mut mb = graphics::MeshBuilder::new();
        for &segment in &self.body {
            mb.rectangle(graphics::DrawMode::fill(), segment.into(), GREEN.into())?;
        }
        mb.rectangle(graphics::DrawMode::fill(), self.head.into(), GREEN.into())?;
        let mesh = mb.build(ctx)?;
        graphics::draw(ctx, &mesh, ggez::graphics::DrawParam::default())
    }
}

struct Game {
    snake: Snake,
    fruit: Fruit,
    x: u32,
}

impl Game {
    fn new() -> Self {
        Self {
            snake: Snake::new(SNAKE_INIT_POS.0, SNAKE_INIT_POS.1),
            fruit: Fruit::new(FRUIT_INIT_POS.0, FRUIT_INIT_POS.1),
            x: 0,
        }
    }

    fn gameover(ctx: &mut Context) {
        event::quit(ctx)
    }
}

impl event::EventHandler for Game {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        self.fruit.draw(ctx)?;
        self.snake.draw(ctx)?;

        graphics::present(ctx)
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, DEFAULT_FPS as u32) {
            self.x = (self.x + 1) % 10;
            if self.x == 0 {
                match self.snake.state {
                    Some(SnakeState::AteFruit) => self.fruit.regenerate(),
                    Some(SnakeState::SelfCollision) => self.snake.reset(),
                    _ => (),
                }
                self.snake.update(&self.fruit)?;
            }
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool
    ) {
        if keycode == KeyCode::Escape {
            Game::gameover(ctx);
        }
        if let Some(direction) = Direction::from_keycode(keycode) {
            if is_opposite(direction, self.snake.direction) {
                self.snake.reverse()
            }
            self.snake.direction = direction;
        }
    }
}

pub fn main() -> GameResult {
    ggez::start(
        ggez::conf::Conf::default()
            .window_title("Snake Game".to_owned())
            .window_width(SCREEN_SIZE.0)
            .window_height(SCREEN_SIZE.1)
        ,
        |_| Box::new(Game::new()),
    )
}