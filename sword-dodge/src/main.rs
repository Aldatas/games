use macroquad::prelude::*;

const PLAYER_SPEED: f32 = 300.0;
const PLAYER_RADIUS: f32 = 12.0;

struct Game {
    player: Vec2,
    swords: Vec<Sword>,
}

struct Sword {
    position: Vec2,
    speed: f32,
}

impl Game {
    fn new() -> Self {
        Self {
            player: vec2(screen_width() / 2.0, screen_height() - 40.0),
            swords: vec![
                Sword {
                    position: vec2(80.0, -40.0),
                    speed: 180.0,
                },
                Sword {
                    position: vec2(200.0, -180.0),
                    speed: 220.0,
                },
                Sword {
                    position: vec2(320.0, -320.0),
                    speed: 260.0,
                },
            ],
        }
    }

    fn update(&mut self, dt: f32) {
        // player
        let mut direction = 0.0;

        if is_key_down(KeyCode::A) {
            direction -= 1.0;
        }

        if is_key_down(KeyCode::D) {
            direction += 1.0;
        }

        self.player.x += direction * PLAYER_SPEED * dt;

        self.player.x = self
            .player
            .x
            .clamp(PLAYER_RADIUS, screen_width() - PLAYER_RADIUS);

        // swords
        for sword in &mut self.swords {
            sword.position.y += sword.speed * dt;

            if sword.position.y > screen_height() + 20.0 {
                sword.position.y = -20.0;
            }
        }
    }

    fn draw(&self) {
        clear_background(BLACK);

        // player
        draw_circle(self.player.x, self.player.y, PLAYER_RADIUS, VIOLET);

        // swords
        for sword in &self.swords {
            let x = sword.position.x;
            let y = sword.position.y;

            // blade
            draw_line(x, y - 15.0, x, y + 15.0, 3.0, SKYBLUE);

            // guard
            draw_line(x - 6.0, y - 10.0, x + 6.0, y - 10.0, 3.0, VIOLET);
        }
    }
}

#[macroquad::main("Sword Dodge")]
async fn main() {
    let mut game = Game::new();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let dt = get_frame_time().min(0.05);

        game.update(dt);
        game.draw();

        next_frame().await;
    }
}
