use macroquad::prelude::*;

const BALL_SIZE: f32 = 10.0;
const BALL_COLOR: Color = MAROON;
const INITIAL_BALL_SPEED: f32 = 150.0;

const PADDLE_HEIGHT: f32 = 8.0;
const INITIAL_PADDLE_WIDTH: f32 = 150.0;
const PADDLE_SPEED: f32 = 720.0;
const PADDLE_COLOR: Color = LIME;

const BRICK_COL_COUNT: usize = 12;
const BRICK_ROW_COUNT: usize = 16;
const BRICK_WIDTH: f32 = 60.0;
const BRICK_HEIGHT: f32 = 20.0;
const BRICK_GAP: f32 = 2.0;
const BRICK_COLOR: Color = BROWN;
const HPADDING: f32 = 28.0;
const VPADDING: f32 = 35.0;

const SCORE_FONT_SIZE: f32 = 20.0;
const SCORE_COLOR: Color = WHITE;

struct Ball {
    pub rect: Rect,
    pub speed: f32,
    pub dir: Vec2,
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            rect: Rect::new(
                screen_width() / 2.0,
                screen_height() / 1.2,
                BALL_SIZE,
                BALL_SIZE,
            ),
            speed: INITIAL_BALL_SPEED,
            dir: vec2(1.0, -1.0),
        }
    }
}

struct Paddle {
    rect: Rect,
}

impl Default for Paddle {
    fn default() -> Self {
        Self {
            rect: Rect::new(
                screen_width() / 2.0,
                screen_height() / 1.05,
                INITIAL_PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
        }
    }
}

#[derive(Clone, Copy)]
struct Brick {
    pub rect: Rect,
    pub is_active: bool,
}

impl Brick {
    fn new() -> Self {
        Self {
            rect: Rect::new(0.0, 0.0, BRICK_WIDTH, BRICK_HEIGHT),
            is_active: true,
        }
    }
}

struct GameState {
    pub ball: Ball,
    pub paddle: Paddle,
    pub bricks: [[Brick; BRICK_COL_COUNT]; BRICK_ROW_COUNT],
    pub is_running: bool,
    pub score: u64,
}

impl Default for GameState {
    fn default() -> Self {
        let mut bricks = [[Brick::new(); BRICK_COL_COUNT]; BRICK_ROW_COUNT];
        for (i, row) in bricks.iter_mut().enumerate() {
            for (j, brick) in row.iter_mut().enumerate() {
                brick.rect.x = j as f32 * (BRICK_WIDTH + BRICK_GAP) + HPADDING;
                brick.rect.y = i as f32 * (BRICK_HEIGHT + BRICK_GAP) + VPADDING;
            }
        }

        Self {
            ball: Ball::default(),
            paddle: Paddle::default(),
            bricks,
            is_running: true,
            score: 0,
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
        window_resizable: false,
        sample_count: 4,
        high_dpi: true,
        ..Default::default()
    }
}

fn handle_input(state: &mut GameState) {
    let GameState { paddle, .. } = state;
    let dt = get_frame_time();

    if is_key_pressed(KeyCode::Backslash) {
        for row in state.bricks.iter_mut().skip(1) {
            for brick in row.iter_mut() {
                brick.is_active = false;
                state.score += 1000;
            }
        }
    }

    if is_key_pressed(KeyCode::Escape) {
        state.is_running = false;
    }

    if is_key_down(KeyCode::D) && paddle.rect.right() <= screen_width() {
        paddle.rect.x += PADDLE_SPEED * dt;
    } else if is_key_down(KeyCode::A) && paddle.rect.x >= 0.0 {
        paddle.rect.x -= PADDLE_SPEED * dt;
    }
}

fn update(state: &mut GameState) {
    let GameState { ball, paddle, .. } = state;
    let dt = get_frame_time();

    paddle.rect.w = INITIAL_PADDLE_WIDTH - INITIAL_PADDLE_WIDTH * state.score as f32 / 256000.0;

    if ball.rect.x <= 0.0 {
        ball.rect.x = 0.0;
        ball.dir.x = 1.0;
    } else if ball.rect.x >= screen_width() {
        ball.rect.x = screen_width();
        ball.dir.x = -1.0;
    }

    if ball.rect.y <= 0.0 {
        ball.rect.y = 0.0;
        ball.dir.y = 1.0;
    } else if ball.rect.y >= screen_height() {
        state.is_running = false;
    }

    ball.rect.x += ball.speed * ball.dir.x * dt;
    ball.rect.y += ball.speed * ball.dir.y * dt;

    ball.speed = INITIAL_BALL_SPEED + INITIAL_BALL_SPEED * state.score as f32 / 76800.0;

    if paddle.rect.overlaps(&ball.rect) {
        ball.dir.y = -1.0;
    }

    for row in state.bricks.iter_mut() {
        for brick in row.iter_mut() {
            if !brick.is_active {
                continue;
            }

            if ball.rect.overlaps(&brick.rect) {
                brick.is_active = false;
                state.score += 1000;

                let ball_middle = vec2(
                    ball.rect.x + ball.rect.w / 2.0,
                    ball.rect.y + ball.rect.h / 2.0,
                );
                let brick_middle = vec2(
                    brick.rect.x + brick.rect.w / 2.0,
                    brick.rect.y + brick.rect.h / 2.0,
                );

                let from_ball_to_brick = ball_middle - brick_middle;
                // is the vector from the ball to the brick more parralel to the up vector (vertical hit)
                // or more perpendicular to the up vector (horizontal hit)
                if from_ball_to_brick.normalize().dot(Vec2::Y).abs() >= 0.25 {
                    ball.dir.y *= -1.0;
                } else {
                    ball.dir.x *= -1.0;
                }
                break;
            }
        }
    }
}

fn draw(state: &GameState) {
    let GameState {
        ball,
        paddle,
        bricks,
        ..
    } = state;

    draw_text(
        &state.score.to_string(),
        screen_width() / 2.0,
        20.0,
        SCORE_FONT_SIZE,
        SCORE_COLOR,
    );

    draw_circle(ball.rect.x, ball.rect.y, BALL_SIZE / 2.0, BALL_COLOR);
    draw_rectangle(
        paddle.rect.x,
        paddle.rect.y,
        paddle.rect.w,
        PADDLE_HEIGHT,
        PADDLE_COLOR,
    );

    for row in bricks.iter() {
        for brick in row.iter() {
            if !brick.is_active {
                continue;
            }

            draw_rectangle(
                brick.rect.x,
                brick.rect.y,
                BRICK_WIDTH,
                BRICK_HEIGHT,
                BRICK_COLOR,
            );
        }
    }
}
