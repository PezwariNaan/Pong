use tetra::graphics::{self, Color, Texture, DrawParams, Rectangle};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};
use std::f32::consts::PI;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED: f32 = 5.0;

struct Player {
    texture: Texture,
    params: DrawParams,
}

struct Ball {
    texture: Texture, 
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

struct GameState {
    player1: Player,
    player2: Player,
    ball: Ball,
}

// First game random player starts. 
// After that winner of previous game starts 

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player1_texture = Texture::new(ctx, "resources/png/paddleBlu.png")?;
        let player1_position = DrawParams::new()
            .position(Vec2::new(45.0, WINDOW_HEIGHT / 2.0 - player1_texture.height() as f32))
            .rotation(PI /2.0);

        let player2_texture = Texture::new(ctx, "resources/png/paddleRed.png")?;
        let player2_position = DrawParams::new()
            .position(Vec2::new(WINDOW_WIDTH - 45.0, WINDOW_HEIGHT / 2.0 - player2_texture.height() as f32))
            .rotation(PI /2.0);

        let ball_texture = Texture::new(ctx, "resources/png/ballGrey.png")?;
        let ball_position = Vec2::new(
                WINDOW_WIDTH  / 2.0 - ball_texture.width()  as f32 / 2.0,
                WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0
            );
        let ball_velocity = Vec2::new(BALL_SPEED, 0.0);

        Ok(GameState {
            player1: Player::new(player1_texture, player1_position),
            player2: Player::new(player2_texture, player2_position),
            ball: Ball::new(ball_texture, ball_position, ball_velocity),
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.8, 0.4, 0.1));
        
        self.player1.texture.draw(ctx, self.player1.params.clone());
        self.player2.texture.draw(ctx, self.player2.params.clone());
        self.ball.texture.draw(ctx, self.ball.position);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::W) {
            self.player1.params.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::S) {
            self.player1.params.position.y += PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Up) {
            self.player2.params.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Down) {
            self.player2.params.position.y += PADDLE_SPEED;
        }

        let player1_bounds = self.player1.bounds();
        let player2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();

        let paddle_hit= if ball_bounds.intersects(&player1_bounds) {
            Some(&self.player1)
        } else if ball_bounds.intersects(&player2_bounds) {
            Some(&self.player2)
        } else {
            None
        };

        if paddle_hit.is_some() {
            self.ball.velocity.x = -self.ball.velocity.x;
        }

        self.ball.position += self.ball.velocity;
        Ok(())
    }
}

impl Player {
    fn new(texture:Texture, params:DrawParams) -> Player {
        Player{texture, params}
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.params.position.x - self.texture.height() as f32,
            self.params.position.y,
            self.texture.height() as f32,
            self.texture.width() as f32,
        )
    }
}

impl Ball {
    fn new(texture:Texture, position:Vec2<f32>, velocity:Vec2<f32>) -> Ball {
        Ball{texture, position, velocity}
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.texture.width() as f32,
            self.texture.height() as f32,
        )
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32,  WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

