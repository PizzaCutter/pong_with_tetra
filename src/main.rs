use tetra::graphics::{self, Texture,  Color, DrawParams};
use tetra::{Context, ContextBuilder, State};
use tetra::math::Vec2;
use tetra::input::{self, Key};

struct Paddle {
    position : Vec2<f32>,
    speed : f32,
    texture: Texture,
}

impl Paddle
{
    fn Draw(&mut self, ctx: &mut Context) -> tetra::Result
    {
        graphics::draw(
            ctx,
            &self.texture,
            DrawParams::new()
                .position(Vec2::new(self.position.x, self.position.y))
                .origin(Vec2::new(8.0, 8.0))
                .scale(Vec2::new(2.0, 2.0)),
        );
        Ok(())
    }
}

struct GameState {
    paddle_1: Paddle,
    //paddle_2: Paddle,
}

impl GameState {
    fn new(context: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            //texture: ,
            paddle_1 : Paddle{
                position: Vec2::new(32.0, 32.0),
                speed: 1.0,
                texture: Texture::new(context, "./resources/player.png")?,
            },
            //paddle_2:position: Vec2::new(400.0, 32.0),
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        //self.player_pos_x += 1.0;
        //self.player_pos_y += 1.0;

        if input::is_key_down(ctx, Key::S)
        {
            self.paddle_1.position.y += self.paddle_1.speed;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Cornflower blue, as is tradition
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.paddle_1.Draw(ctx);
        //self.paddle_2.Draw(ctx);
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Hello, world!", 1280, 720)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
