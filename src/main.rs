use macroquad::prelude::*;

#[macroquad::main(window_config)]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);

        handle_input();

        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 100.0, RED);

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

fn handle_input() {
    if is_key_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}
