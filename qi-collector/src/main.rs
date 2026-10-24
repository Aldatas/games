use macroquad::prelude::*;

const SCREEN_SIZE: i32 = 400;
const PLAYER_RADIUS: f32 = 16.0;
const ORB_RADIUS: f32 = 10.0;
const SPEED: f32 = 250.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Qi Collector".to_owned(),
        window_width: SCREEN_SIZE,
        window_height: SCREEN_SIZE,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = vec2(300.0, 225.0);
    let mut orb = vec2(200.0, 150.0);
    let mut score = 0;

    loop {
        let mut direction = Vec2::ZERO;

        if is_key_down(KeyCode::Escape) {
            break;
        }
        if is_key_down(KeyCode::W) {
            direction.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            direction.y += 1.0;
        }
        if is_key_down(KeyCode::A) {
            direction.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            direction.x += 1.0;
        }

        player += direction.normalize_or_zero() * SPEED * get_frame_time();

        let max = SCREEN_SIZE as f32 - PLAYER_RADIUS;

        player.x = player.x.clamp(PLAYER_RADIUS, max);
        player.y = player.y.clamp(PLAYER_RADIUS, max);

        if player.distance(orb) < PLAYER_RADIUS + ORB_RADIUS {
            score += 1;
            orb.x = rand::gen_range(30.0, 370.0);
            orb.y = rand::gen_range(30.0, 370.0);
        }

        // Draw
        clear_background(BLACK);

        draw_circle(player.x, player.y, PLAYER_RADIUS, VIOLET);
        draw_circle(orb.x, orb.y, ORB_RADIUS, SKYBLUE);

        draw_text(&format!("Qi: {score}"), 20.0, 30.0, 24.0, WHITE);

        next_frame().await;
    }
}
