use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};
use std::f32::consts::PI;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED: f32 = 8.0;

struct GameState {
    paddle_texture: Texture,
    paddle_position: graphics::DrawParams,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let paddle_texture = Texture::new(ctx, "resources/png/paddleBlu.png")?;
        let paddle_position = graphics::DrawParams::new()
            .position(Vec2::new(45.0, WINDOW_HEIGHT / 2.0 - paddle_texture.height() as f32))
            .rotation(PI /2.0);

        Ok(GameState {
            paddle_texture,
            paddle_position,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.8, 0.4, 0.1));
        
        self.paddle_texture.draw(ctx, self.paddle_position.clone());

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32,  WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

