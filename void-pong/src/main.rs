use std::{arch::x86_64, collections::btree_map::Keys};

use macroquad::prelude::*;

#[macroquad::main("Void Pong")]
async fn main() {
    let mut ball = vec2(200.0, 200.0);
    let mut velocity = vec2(180.0, 130.0);
    let radius = 12.0;

    let mut paddle = Rect::new(
        screen_width() / 2.0 - 45.0,
        screen_height() - 40.0,
        90.0,
        12.0,
    );

    let paddle_speed = 350.0;
    let mut score = 0;
    let mut speed_amplifier = 1.0;

    loop {
        let dt = get_frame_time().min(0.05);

        // input
        if is_key_down(KeyCode::Escape) {
            break;
        }
        if is_key_down(KeyCode::A) {
            paddle.x -= paddle_speed * dt;
        }
        if is_key_down(KeyCode::D) {
            paddle.x += paddle_speed * dt;
        }

        paddle.x = paddle.x.clamp(0.0, screen_width() - paddle.w);

        // velocity
        ball += velocity * dt * speed_amplifier;

        if ball.x <= radius {
            ball.x = radius;
            velocity.x = velocity.x.abs();
        } else if ball.x >= screen_width() - radius {
            ball.x = screen_width() - radius;
            velocity.x = -velocity.x.abs();
        }

        if ball.y <= radius {
            ball.y = radius;
            velocity.y = velocity.y.abs();
        } else if ball.y >= screen_height() - radius {
            ball.y = screen_height() - radius;
            velocity.y = -velocity.y.abs();
        }

        let closest_x = ball.x.clamp(paddle.x, paddle.x + paddle.w);
        let closest_y = ball.y.clamp(paddle.y, paddle.y + paddle.h);

        let dx = ball.x - closest_x;
        let dy = ball.y - closest_y;

        let colliding = dx * dx + dy * dy <= radius * radius;

        if colliding && velocity.y > 0.0 && ball.y <= paddle.y {
            ball.y = paddle.y - radius;
            velocity.y = -velocity.y.abs();
            score += 1;
            speed_amplifier += 0.1;
        }

        // rendering
        clear_background(BLACK);
        draw_rectangle(paddle.x, paddle.y, paddle.w, paddle.h, SKYBLUE);
        draw_circle(ball.x, ball.y, radius, VIOLET);
        draw_text(&format!("{score}"), 20.0, 30.0, 24.0, WHITE);

        next_frame().await;
    }
}
