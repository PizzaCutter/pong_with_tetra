use tetra::graphics::{self, Texture,  Color, DrawParams};
use tetra::{Context, ContextBuilder, State};
use tetra::math::Vec2;
use tetra::input::{self, Key};
use std::time::{SystemTime};

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const PADDLE_OFFSET: f32 = 50.0;

struct Paddle {
    position : Vec2<f32>,
    speed : f32,
    texture: Texture,
}

impl Paddle
{
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result
    {
        graphics::draw(
            ctx,
            &self.texture,
            DrawParams::new()
                .position(Vec2::new(self.position.x, self.position.y))
                .origin(Vec2::new(12.5, 50.0))
                .scale(Vec2::new(1.0, 1.0)),
        );
        Ok(())
    }

    fn movement(&mut self, axis_value : f32, delta_time : f32)
    {
        self.position.y += (self.speed * axis_value) * delta_time;
    }
}

struct GameState {
    paddle_1: Paddle,
    paddle_2: Paddle,
    delta_time: f32,
    prev_frame_time: SystemTime,
}

impl GameState {
    fn new(context: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            paddle_1 : Paddle{
                position: Vec2::new(PADDLE_OFFSET,  WINDOW_HEIGHT / 2.0),
                speed: 200.0, 
                texture: Texture::new(context, "./resources/paddle.png")?,
            },
            paddle_2: Paddle {
                position: Vec2::new(WINDOW_WIDTH - PADDLE_OFFSET, WINDOW_HEIGHT / 2.0),
                speed: 200.0,
                texture: Texture::new(context, "./resources/paddle.png")?,
            },
            delta_time: 0.16,
            prev_frame_time: SystemTime::now(),
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.prev_frame_time.elapsed() {
            Ok(_elapsed) => {
                let duration_since_last_frame = _elapsed.as_millis() as f32;
                self.delta_time = duration_since_last_frame / 1000.0;
                self.prev_frame_time = SystemTime::now();
                //println!("delta_time: {0}", self.delta_time);
            },
            Err(_e) => println!("Error when getting elapsed time {0}", _e),
        }
        
        if input::is_key_down(ctx, Key::W)
        {
            self.paddle_1.movement(-1.0, self.delta_time);
        }
        if input::is_key_down(ctx, Key::S)
        {
            self.paddle_1.movement(1.0, self.delta_time);
        }

        if input::is_key_down(ctx, Key::Up)
        {
            self.paddle_2.movement(-1.0, self.delta_time);
        }
        if input::is_key_down(ctx, Key::Down)
        {
            self.paddle_2.movement(1.0, self.delta_time);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        match self.paddle_1.draw(ctx) {
            Ok(result)  => result,
            Err(e) => return Err(e),
        };
        match self.paddle_2.draw(ctx) {
            Ok(result)  => result,
            Err(e) => return Err(e),
        };

        return Ok(());
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Hello, world!", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
