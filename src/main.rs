use tetra::graphics::{self, Texture,  Color, DrawParams};
use tetra::{Context, ContextBuilder, State};
use tetra::math::Vec2;
use tetra::input::{self, Key};
use std::time::{SystemTime};
use rand::prelude::*;
use std::f32;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const PADDLE_OFFSET: f32 = 50.0;
const BALL_START_VELOCITY : f32 = 200.0;

struct Rect {
    position : Vec2<f32>,
    dimensions: Vec2<f32>,
}

struct Circle {
    position : Vec2<f32>,
    radius : f32,
}

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

    fn get_rect(&mut self) -> Rect
    {
        return Rect {
            position : self.position,
            dimensions : Vec2::new(self.texture.width() as f32 / 2.0, self.texture.height() as f32 / 2.0),
        };
    }
}

struct Ball {
    position : Vec2<f32>,
    velocity : Vec2<f32>,
    texture: Texture,
}

impl Ball {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result
    {
        graphics::draw(
            ctx,
            &self.texture,
            DrawParams::new()
                .position(Vec2::new(self.position.x, self.position.y))
                .origin(Vec2::new(12.5, 12.5))
                .scale(Vec2::new(1.0, 1.0)),
        );
        Ok(())
    }

    fn update_position(&mut self, delta_time : f32)
    {
        self.position += self.velocity * delta_time;

        if self.position.x < 0.0 || self.position.x > WINDOW_WIDTH
        {
            self.position = Vec2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);
        }

        if self.position.y < 0.0 || self.position.y > WINDOW_HEIGHT 
        {
            self.velocity.y *= -1.0;
        }
    }

    fn check_collision(&mut self, rect : &Rect) -> bool
    {
        let circle = self.get_circle();

        let circle_distance_x = (circle.position.x - rect.position.x).abs();
        let circle_distance_y = (circle.position.y - rect.position.y).abs();

        if circle_distance_x > (rect.dimensions.x / 2.0 + circle.radius)
        {
            return false;
        }
        if circle_distance_y > (rect.dimensions.y / 2.0 + circle.radius)
        {
            return false;
        }

        if circle_distance_x > (rect.dimensions.x / 2.0)
        {
            return true;
        }
        if circle_distance_y > (rect.dimensions.y / 2.0)
        {
            return true;
        }

        let temp = (circle_distance_x - rect.dimensions.x / 2.0).powf(2.0) + (circle_distance_y - rect.dimensions.y / 2.0).powf(2.0);

        return temp <= circle.radius.powf(2.0);
    }

    fn get_circle(&mut self) -> Circle
    {
        return Circle {
            position: self.position,
            radius: self.texture.width() as f32,
        }
    }
}

struct GameState {
    paddle_1: Paddle,
    paddle_2: Paddle,
    ball: Ball,
    delta_time: f32,
    prev_frame_time: SystemTime,
}

impl GameState {
    fn new(context: &mut Context) -> tetra::Result<GameState> {
        let mut rng = rand::thread_rng();
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
            ball: Ball {
                position: Vec2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0),
                velocity: {
                    let mut rnd_y : f32 = rng.gen();
                    rnd_y = (rnd_y * 2.0) - 1.0;
                    if rnd_y < -0.5
                    {
                        rnd_y = -0.5;
                    }
                    else if rnd_y > 0.5
                    {
                        rnd_y = 0.5;
                    }
                    Vec2::new(BALL_START_VELOCITY, rnd_y * BALL_START_VELOCITY)
                },
                texture: Texture::new(context, "./resources/ball.png")?,
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

        if self.ball.check_collision(&self.paddle_1.get_rect()) 
        {
            self.ball.velocity *= -1.0;
        }
        if self.ball.check_collision(&self.paddle_2.get_rect())
        {
            self.ball.velocity *= -1.0;
        }
        self.ball.update_position(self.delta_time);

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
        match self.ball.draw(ctx) {
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
