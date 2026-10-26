use macroquad::prelude::*;

const PLAYER_SPEED: f32 = 300.0;
const PLAYER_RADIUS: f32 = 12.0;

const SWORD_SPAWN_MARGIN: f32 = 20.0;

const RAMP_DURATION: f32 = 33.0;

#[derive(Clone, Copy, PartialEq, Eq)]
enum GameState {
    Playing,
    GameOver,
}

struct Difficulty {
    spawn_interval: f32,
    sword_speed_bonus: f32,
}

impl Difficulty {
    fn at(elapsed: f32) -> Self {
        let progress = (elapsed / RAMP_DURATION).clamp(0.0, 1.0);

        Self {
            spawn_interval: 0.7 - 0.69 * progress,
            sword_speed_bonus: 360.0 * progress,
        }
    }
}

struct Game {
    player: Vec2,
    swords: Vec<Sword>,
    spawn_timer: f32,
    state: GameState,
    survival_time: f32,
}

struct Sword {
    position: Vec2,
    speed: f32,
}

impl Game {
    fn new() -> Self {
        Self {
            player: vec2(screen_width() / 2.0, screen_height() - 40.0),
            swords: Vec::new(),
            spawn_timer: 0.0,
            state: GameState::Playing,
            survival_time: 0.0,
        }
    }

    fn update(&mut self, dt: f32) {
        if self.state == GameState::GameOver {
            return;
        }

        self.survival_time += dt;

        let difficulty = Difficulty::at(self.survival_time);

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
        self.spawn_timer += dt;

        while self.spawn_timer >= difficulty.spawn_interval {
            self.spawn_timer -= difficulty.spawn_interval;

            // let sword = Sword {
            //     position: vec2(
            //         rand::gen_range(SWORD_SPAWN_MARGIN, screen_width() - SWORD_SPAWN_MARGIN),
            //         -20.0,
            //     ),
            //     speed: rand::gen_range(170.0, 200.0),
            // };
            //
            // self.swords.push(sword);

            self.swords.push(Sword {
                position: vec2(
                    rand::gen_range(SWORD_SPAWN_MARGIN, screen_width() - SWORD_SPAWN_MARGIN),
                    -20.0,
                ),
                speed: rand::gen_range(170.0, 280.0) + difficulty.sword_speed_bonus,
            });
        }

        // Move all swords
        for sword in &mut self.swords {
            sword.position.y += sword.speed * dt;
        }

        // Remove swords that leave the screen
        self.swords
            .retain(|sword| sword.position.y <= screen_height() + 20.0);

        // Sword collision
        for sword in &self.swords {
            // Closest point on rect hitbox of sword
            let closest_x = self
                .player
                .x
                .clamp(sword.position.x - 6.0, sword.position.x + 6.0);

            let closest_y = self
                .player
                .y
                .clamp(sword.position.y - 15.0, sword.position.y + 15.0);

            let dx = self.player.x - closest_x;
            let dy = self.player.y - closest_y;

            if dx * dx + dy * dy <= PLAYER_RADIUS * PLAYER_RADIUS {
                self.state = GameState::GameOver;
                break;
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

        match self.state {
            GameState::Playing => {}

            GameState::GameOver => {
                draw_rectangle(
                    0.0,
                    0.0,
                    screen_width(),
                    screen_height(),
                    Color::new(0.0, 0.0, 0.0, 0.8),
                );

                draw_text("TRIBULATION FAILED", 40.0, 180.0, 30.0, RED);
                draw_text("ENTER: Retry", 40.0, 220.0, 24.0, WHITE);
            }
        }

        draw_text(
            &format!("Cultivation: {:.1}s", self.survival_time),
            20.0,
            40.0,
            24.0,
            WHITE,
        );
    }

    fn reset(&mut self) {
        *self = Self::new();
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

        if is_key_pressed(KeyCode::Enter) && game.state == GameState::GameOver {
            game.reset();
        }

        game.update(dt);
        game.draw();

        next_frame().await;
    }
}
