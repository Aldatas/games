use std::{arch::x86_64, collections::btree_map::Keys};

use macroquad::prelude::*;

#[macroquad::main("Void Pong")]
async fn main() {
    let mut ball = vec2(200.0, 200.0);
    let mut velocity = vec2(180.0, 130.0);
    let radius = 12.0;

    loop {
        // input
        if (is_key_down(KeyCode::Escape)) {
            break;
        }

        // velocity

        let dt = get_frame_time();

        ball += velocity * dt;

        if ball.x <= radius || ball.x >= screen_width() - radius {
            velocity.x = -velocity.x;
        }

        if ball.y <= radius || ball.y >= screen_height() - radius {
            velocity.y = -velocity.y;
        }

        clear_background(BLACK);
        draw_circle(ball.x, ball.y, radius, VIOLET);

        next_frame().await;
    }
}
