use lazy_static::lazy_static;
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
const BRICK_HPADDING: f32 = 28.0;
const BRICK_VPADDING: f32 = 35.0;

const SCORE_AT_MIN_PADDLE_WIDTH: f32 = 256000.0;
const SCORE_AT_MAX_BALL_SPEED: f32 = 76800.0;

lazy_static! {
    static ref BRICK_COLORS: [Color; 4] = [
        Color::from_rgba(26, 26, 64, 255),
        Color::from_rgba(39, 0, 130, 255),
        Color::from_rgba(122, 11, 192, 255),
        Color::from_rgba(250, 88, 182, 255),
    ];
}

const SCORE_FONT_SIZE: f32 = 20.0;
const SCORE_COLOR: Color = WHITE;

#[macroquad::main(window_config)]
async fn main() {
    GameState::new().run().await;
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

struct GameState {
    pub ball: Ball,
    pub paddle: Paddle,
    pub bricks: BrickGrid,
    pub is_running: bool,
    pub score: u64,
    pub dt: f32,
}

impl GameState {
    fn new() -> Self {
        Self {
            ball: Ball::default(),
            paddle: Paddle::default(),
            bricks: Brick::make_grid(),
            is_running: false,
            score: 0,
            dt: 0.0,
        }
    }

    async fn run(mut self) {
        self.is_running = true;
        while self.is_running {
            self.handle_input();
            self.update();
            self.draw();

            next_frame().await
        }
    }

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.is_running = false;
        }

        self.handle_paddle_input();
    }

    fn update(&mut self) {
        self.dt = get_frame_time();
        self.paddle.rect.w = INITIAL_PADDLE_WIDTH
            - INITIAL_PADDLE_WIDTH * self.score as f32 / SCORE_AT_MIN_PADDLE_WIDTH;

        self.update_ball_pos();
        self.ball.speed =
            INITIAL_BALL_SPEED + INITIAL_BALL_SPEED * self.score as f32 / SCORE_AT_MAX_BALL_SPEED;
        self.handle_collision();
    }

    fn draw(&self) {
        clear_background(LIGHTGRAY);
        self.draw_score();
        self.draw_ball();
        self.draw_paddle();
        self.draw_bricks();
    }

    fn handle_paddle_input(&mut self) {
        let GameState { paddle, .. } = self;
        if is_key_down(KeyCode::D) && paddle.rect.right() < screen_width() {
            paddle.rect.x += PADDLE_SPEED * self.dt;
        } else if is_key_down(KeyCode::A) && paddle.rect.x > 0.0 {
            paddle.rect.x -= PADDLE_SPEED * self.dt;
        }
    }

    fn update_ball_pos(&mut self) {
        let GameState { ball, .. } = self;
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
            self.is_running = false;
        }
        ball.rect.x += ball.speed * ball.dir.x * self.dt;
        ball.rect.y += ball.speed * ball.dir.y * self.dt;
    }

    fn handle_collision(&mut self) {
        let GameState { ball, paddle, .. } = self;
        if paddle.rect.overlaps(&ball.rect) {
            ball.dir.y = -1.0;
        }

        for row in self.bricks.iter_mut() {
            for brick in row.iter_mut() {
                if !brick.is_active {
                    continue;
                }

                if ball.rect.overlaps(&brick.rect) {
                    brick.is_active = false;
                    self.score += 1000;

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

    fn draw_bricks(&self) {
        for (row_index, row) in self.bricks.iter().enumerate() {
            let brick_color_index = row_index / BRICK_COLORS.len();
            let brick_color = BRICK_COLORS[brick_color_index];

            for brick in row.iter() {
                if !brick.is_active {
                    continue;
                }

                draw_rectangle(
                    brick.rect.x,
                    brick.rect.y,
                    BRICK_WIDTH,
                    BRICK_HEIGHT,
                    brick_color,
                );
            }
        }
    }

    fn draw_paddle(&self) {
        draw_rectangle(
            self.paddle.rect.x,
            self.paddle.rect.y,
            self.paddle.rect.w,
            PADDLE_HEIGHT,
            PADDLE_COLOR,
        );
    }

    fn draw_score(&self) {
        draw_text(
            &self.score.to_string(),
            screen_width() / 2.0,
            20.0,
            SCORE_FONT_SIZE,
            SCORE_COLOR,
        );
    }

    fn draw_ball(&self) {
        draw_circle(
            self.ball.rect.x,
            self.ball.rect.y,
            BALL_SIZE / 2.0,
            BALL_COLOR,
        );
    }
}

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

type BrickGrid = [[Brick; BRICK_COL_COUNT]; BRICK_ROW_COUNT];

impl Brick {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(0.0, 0.0, BRICK_WIDTH, BRICK_HEIGHT),
            is_active: true,
        }
    }

    pub fn make_grid() -> BrickGrid {
        let mut bricks = [[Brick::new(); BRICK_COL_COUNT]; BRICK_ROW_COUNT];
        for (i, row) in bricks.iter_mut().enumerate() {
            for (j, brick) in row.iter_mut().enumerate() {
                brick.rect.x = j as f32 * (BRICK_WIDTH + BRICK_GAP) + BRICK_HPADDING;
                brick.rect.y = i as f32 * (BRICK_HEIGHT + BRICK_GAP) + BRICK_VPADDING;
            }
        }
        bricks
    }
}
