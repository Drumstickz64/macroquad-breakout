use macroquad::prelude::*;

const BALL_RADIUS: f32 = 10.0;
const BALL_COLOR: Color = MAROON;

const PADDLE_HEIGHT: f32 = 12.0;
const PADDLE_SPEED: f32 = 650.0;
const PADDLE_COLOR: Color = LIME;

struct Ball {
    pub pos: Vec2,
    pub speed: f32,
    pub dir: Vec2,
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            pos: Vec2::new(screen_width() / 2.0, screen_height() / 1.2),
            speed: 240.0,
            dir: Vec2::new(1.0, -1.0),
        }
    }
}

struct Paddle {
    pos: Vec2,
    width: f32,
}

impl Default for Paddle {
    fn default() -> Self {
        Self {
            pos: Vec2::new(screen_width() / 2.0, screen_height() / 1.05),
            width: 200.0,
        }
    }
}

struct GameState {
    pub ball: Ball,
    pub paddle: Paddle,
    pub dt: f32,
    pub is_running: bool,
    last_update: f64,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            ball: Ball::default(),
            paddle: Paddle::default(),
            dt: 0.0,
            is_running: true,
            last_update: 0.0,
        }
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut state = GameState::default();

    while state.is_running {
        clear_background(LIGHTGRAY);

        handle_input(&mut state);
        update(&mut state);
        draw(&state);

        next_frame().await
    }
}

fn window_config() -> Conf {
    Conf {
        window_title: "Breakout".to_string(),
        sample_count: 4,
        high_dpi: true,
        ..Default::default()
    }
}

fn handle_input(state: &mut GameState) {
    let GameState { paddle, dt, .. } = state;

    if is_key_pressed(KeyCode::Escape) {
        state.is_running = false;
    }

    if is_key_down(KeyCode::D) {
        paddle.pos.x += PADDLE_SPEED * *dt;
    } else if is_key_down(KeyCode::A) {
        paddle.pos.x -= PADDLE_SPEED * *dt;
    }
}

fn update(state: &mut GameState) {
    let GameState { ball, dt, .. } = state;

    *dt = (get_time() - state.last_update) as f32;
    state.last_update = get_time();

    if ball.pos.x <= 0.0 {
        ball.pos.x = 0.0;
        ball.dir.x = 1.0;
    } else if ball.pos.x >= screen_width() {
        ball.pos.x = screen_width();
        ball.dir.x = -1.0;
    }

    ball.pos += ball.speed * ball.dir * *dt;
}

fn draw(state: &GameState) {
    draw_circle(state.ball.pos.x, state.ball.pos.y, BALL_RADIUS, BALL_COLOR);
    draw_rectangle(
        state.paddle.pos.x,
        state.paddle.pos.y,
        state.paddle.width,
        PADDLE_HEIGHT,
        PADDLE_COLOR,
    )
}
