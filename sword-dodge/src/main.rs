use macroquad::prelude::*;

const PLAYER_SPEED: f32 = 300.0;
const PLAYER_RADIUS: f32 = 1.0;
const PLAYER_HALF_WIDTH: f32 = 16.0;

const SWORD_SPAWN_MARGIN: f32 = 20.0;

const RAMP_DURATION: f32 = 33.0;
const ENDGAME_RAMP_DURATION: f32 = 16.0;
const ULTRA_RAMP_DURATION: f32 = 7.0;
const RAID_RAMP_DURATION: f32 = 4.0;

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

        let endgame = ((elapsed - RAMP_DURATION) / ENDGAME_RAMP_DURATION).clamp(0.0, 1.0);
        let ultra_start = RAMP_DURATION + ENDGAME_RAMP_DURATION;
        let raid =
            ((elapsed - ultra_start - ULTRA_RAMP_DURATION) / RAID_RAMP_DURATION).clamp(0.0, 1.0);

        Self {
            spawn_interval: (0.7 - 0.6 * progress - 0.05 * endgame)
                * (1.0 - 0.985 * raid).max(0.015),
            sword_speed_bonus: 360.0 * progress + 240.0 * endgame + 500.0 * raid,
        }
    }
}

struct Game {
    player: Vec2,
    swords: Vec<Sword>,
    sparks: Vec<Spark>,
    spawn_timer: f32,
    state: GameState,
    survival_time: f32,
    god_mode: bool,
}

struct Spark {
    position: Vec2,
    velocity: Vec2,
    life: f32,
    max_life: f32,
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
            sparks: Vec::new(),
            spawn_timer: 0.0,
            state: GameState::Playing,
            survival_time: 0.0,
            god_mode: false,
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
            .clamp(PLAYER_HALF_WIDTH, screen_width() - PLAYER_HALF_WIDTH);

        // swords
        self.spawn_timer += dt;

        // Raid safe-path variables
        let raid_start_time = RAMP_DURATION + ENDGAME_RAMP_DURATION + ULTRA_RAMP_DURATION;
        let raid_progress =
            ((self.survival_time - raid_start_time) / RAID_RAMP_DURATION).clamp(0.0, 1.0);

        let mut spawned = 0;
        while self.spawn_timer >= difficulty.spawn_interval && spawned < 80 {
            self.spawn_timer -= difficulty.spawn_interval;
            spawned += 1;

            let mut spawn_x =
                rand::gen_range(SWORD_SPAWN_MARGIN, screen_width() - SWORD_SPAWN_MARGIN);

            // Create meandering safe tunnels during the Raid phase
            if raid_progress > 0.0 {
                let max_hole_width = 55.0; // Radius of the tunnel (110px total width)
                let hole_width = max_hole_width * raid_progress;

                let t = self.survival_time;
                let sw = screen_width();

                // Path 1
                let p1 = sw * 0.5 + (t * 0.85).sin() * (sw * 0.3) + (t * 1.3).cos() * (sw * 0.15);

                // Path 2 (Different frequencies so they weave and cross independently)
                let p2 = sw * 0.5
                    + (t * 1.1 + 4.0).sin() * (sw * 0.35)
                    + (t * 0.7 + 2.0).cos() * (sw * 0.1);

                // Push swords out of Path 1
                if (spawn_x - p1).abs() < hole_width {
                    spawn_x = if spawn_x < p1 {
                        p1 - hole_width
                    } else {
                        p1 + hole_width
                    };
                }

                // Push swords out of Path 2
                if (spawn_x - p2).abs() < hole_width {
                    spawn_x = if spawn_x < p2 {
                        p2 - hole_width
                    } else {
                        p2 + hole_width
                    };
                }

                // Ensure swords pushed by tunnels don't stick out of bounds
                spawn_x = spawn_x.clamp(SWORD_SPAWN_MARGIN, sw - SWORD_SPAWN_MARGIN);
            }

            self.swords.push(Sword {
                position: vec2(spawn_x, -40.0),
                speed: rand::gen_range(170.0, 280.0) + difficulty.sword_speed_bonus,
            });
        }

        // Move all swords
        for sword in &mut self.swords {
            sword.position.y += sword.speed * dt;
        }

        // The blade tip strikes the bottom edge once, throwing a small fountain upward.
        let ground = screen_height();
        for sword in &self.swords {
            if sword.position.y + 15.0 >= ground {
                for i in 0..7 {
                    let spread = (i as f32 - 3.0) / 3.0;
                    let life = 0.32 + (1.0 - spread.abs()) * 0.16;
                    self.sparks.push(Spark {
                        position: vec2(sword.position.x, ground - 1.0),
                        velocity: vec2(spread * 120.0, -120.0 - (1.0 - spread.abs()) * 90.0),
                        life,
                        max_life: life,
                    });
                }
            }
        }
        self.swords.retain(|sword| sword.position.y + 15.0 < ground);
        for spark in &mut self.sparks {
            spark.position += spark.velocity * dt;
            spark.velocity.y += 400.0 * dt; // Gravity pull
            spark.life -= dt;
        }
        self.sparks.retain(|spark| spark.life > 0.0);

        // Sword collision
        if !self.god_mode {
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
    }

    fn draw(&self, cultivator: &Texture2D) {
        // Keep it pitch black base
        clear_background(BLACK);

        let intensity =
            ((self.survival_time - RAMP_DURATION) / ENDGAME_RAMP_DURATION).clamp(0.0, 1.0);
        let ultra = ((self.survival_time - RAMP_DURATION - ENDGAME_RAMP_DURATION)
            / ULTRA_RAMP_DURATION)
            .clamp(0.0, 1.0);
        let raid =
            ((self.survival_time - RAMP_DURATION - ENDGAME_RAMP_DURATION - ULTRA_RAMP_DURATION)
                / RAID_RAMP_DURATION)
                .clamp(0.0, 1.0);

        // Only draw the background after reaching endgame, fading it in using "intensity"
        if self.survival_time >= RAMP_DURATION {
            draw_cosmos(get_time() as f32, intensity, ultra, raid);
        }

        // Draw Player
        draw_texture_ex(
            cultivator,
            self.player.x - 16.0,
            self.player.y - 20.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 40.0)),
                ..Default::default()
            },
        );

        // Player Aura / Core
        let pulse = (get_time() as f32 * 8.0).sin() * 0.5 + 0.5;
        draw_circle(
            self.player.x,
            self.player.y,
            8.0 + pulse * 2.0,
            Color::new(0.5, 0.8, 1.0, 0.15),
        );
        draw_circle(
            self.player.x,
            self.player.y,
            4.0,
            Color::new(0.65, 0.9, 1.0, 0.6),
        );
        draw_circle(self.player.x, self.player.y, PLAYER_RADIUS, WHITE);

        // Swords
        for sword in &self.swords {
            let x = sword.position.x;
            let y = sword.position.y;

            // Subtler, shorter motion trail
            let trail_length = (sword.speed * 0.05).clamp(10.0, 25.0);
            draw_line(
                x,
                y - 15.0,
                x,
                y - 15.0 - trail_length,
                3.0,
                Color::new(0.0, 0.6, 1.0, 0.1), // Faint blue
            );

            // Outer Glow
            draw_line(
                x,
                y - 15.0,
                x,
                y + 15.0,
                5.0,
                Color::new(0.1, 0.5, 0.9, 0.3),
            );

            // Inner Core (Blade)
            draw_line(
                x,
                y - 15.0,
                x,
                y + 15.0,
                3.0,
                Color::new(0.7, 0.95, 1.0, 0.9),
            );
            draw_line(x, y - 14.0, x, y + 14.0, 1.0, WHITE);

            // Blade Tip (Sharp point)
            draw_triangle(
                vec2(x - 1.5, y + 15.0),
                vec2(x + 1.5, y + 15.0),
                vec2(x, y + 20.0),
                WHITE,
            );
            draw_triangle(
                // Tip glow
                vec2(x - 3.5, y + 15.0),
                vec2(x + 3.5, y + 15.0),
                vec2(x, y + 23.0),
                Color::new(0.1, 0.5, 0.9, 0.4),
            );

            // Guard Outer Glow
            draw_line(
                x - 8.0,
                y - 9.0,
                x + 8.0,
                y - 9.0,
                5.0,
                Color::new(0.6, 0.1, 0.8, 0.5),
            );
            // Guard Inner Core
            draw_line(
                x - 7.0,
                y - 9.0,
                x + 7.0,
                y - 9.0,
                2.0,
                Color::new(0.9, 0.5, 1.0, 1.0),
            );

            // Pommel / Handle
            draw_line(
                x,
                y - 10.0,
                x,
                y - 15.0,
                2.0,
                Color::new(0.3, 0.1, 0.4, 1.0),
            );
        }

        // Sparks
        for spark in &self.sparks {
            let life_ratio = (spark.life / spark.max_life).clamp(0.0, 1.0);

            // Color shifts from hot white to cyan to purple as it dies
            let color = if life_ratio > 0.6 {
                Color::new(1.0, 0.9, 0.8, life_ratio)
            } else if life_ratio > 0.3 {
                Color::new(0.4, 0.8, 1.0, life_ratio)
            } else {
                Color::new(0.5, 0.2, 0.8, life_ratio)
            };

            let length_mult = 0.04 + (life_ratio * 0.04);
            draw_line(
                spark.position.x,
                spark.position.y,
                spark.position.x - spark.velocity.x * length_mult,
                spark.position.y - spark.velocity.y * length_mult,
                1.0 + life_ratio * 2.0,
                color,
            );
        }

        match self.state {
            GameState::Playing => {}
            GameState::GameOver => {
                draw_rectangle(
                    0.0,
                    0.0,
                    screen_width(),
                    screen_height(),
                    Color::new(0.0, 0.0, 0.0, 0.85),
                );

                let text_size = 40.0;
                let text = "TRIBULATION FAILED";
                let text_dims = measure_text(text, None, text_size as _, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dims.width / 2.0,
                    screen_height() / 2.0 - 20.0,
                    text_size,
                    RED,
                );

                let sub_text_size = 24.0;
                let sub_text = "SPACE: Retry";
                let sub_text_dims = measure_text(sub_text, None, sub_text_size as _, 1.0);
                draw_text(
                    sub_text,
                    screen_width() / 2.0 - sub_text_dims.width / 2.0,
                    screen_height() / 2.0 + 20.0,
                    sub_text_size,
                    Color::new(0.8, 0.8, 0.8, 1.0),
                );
            }
        }

        draw_text(
            &format!("Cultivation: {:.1}s", self.survival_time),
            20.0,
            40.0,
            24.0,
            Color::new(0.9, 0.9, 1.0, 0.8),
        );

        if self.god_mode {
            draw_text(
                "GODMODE ON",
                20.0,
                80.0,
                12.0,
                Color::new(0.7, 0.1, 0.7, 0.7),
            );
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }
}

// Highly upgraded deterministic background
fn draw_cosmos(time: f32, intensity: f32, ultra: f32, raid: f32) {
    let width = screen_width();
    let height = screen_height();

    // Parallax Star Layers with randomized paths
    for layer in 0..4 {
        // Closer layers move significantly faster
        let base_speed =
            (15.0 + layer as f32 * 25.0) * (1.0 + intensity * 0.8 + ultra * 1.5 + raid * 5.0);
        let star_count = (40 + layer * 15) as i32;

        for i in 0..star_count {
            // Seed provides pseudo-randomness for each specific star
            let seed = (i * 97 + layer * 173) as f32;

            // Randomize speed multiplier per star (0.5x to 1.5x)
            let speed_mult = 0.5 + (seed % 10.0) / 10.0;
            let speed = base_speed * speed_mult;

            // Give each star a unique trajectory angle (-0.4 to 0.4 radians off straight down)
            let angle = ((seed % 11.0) / 11.0 - 0.5) * 0.8;
            let dx = angle.sin() * speed;
            let dy = angle.cos() * speed;

            let x = (seed * 53.17 + time * dx).rem_euclid(width.max(1.0));
            let y = (seed * 89.31 + time * dy).rem_euclid(height.max(1.0));

            let twinkle = 0.5 + 0.5 * (time * (2.0 + layer as f32) + seed).sin();
            let base_alpha = 0.1 + layer as f32 * 0.15;
            // Fade stars in cleanly via "intensity"
            let alpha = base_alpha * (1.0 + intensity * 0.5) * twinkle * intensity;

            // Shift star colors depending on the layer
            let color = match layer {
                0 => Color::new(0.3, 0.1, 0.6, alpha), // deep backdrop
                1 => Color::new(0.2, 0.4, 0.8, alpha),
                2 => Color::new(0.6, 0.8, 1.0, alpha),
                _ => Color::new(0.9, 0.95, 1.0, alpha), // foreground
            };

            let size = 0.5 + layer as f32 * 0.6;
            draw_circle(x, y, size, color);

            // Motion blur traces along the unique angle vector of that star
            if layer >= 2 && (ultra > 0.0 || raid > 0.0) {
                let stretch = speed * 0.04;
                draw_line(
                    x - angle.sin() * stretch,
                    y - angle.cos() * stretch,
                    x,
                    y,
                    size,
                    Color::new(color.r, color.g, color.b, alpha * 0.6),
                );
            }
        }
    }

    // Ultra mode: Golden/Purple cosmic geometry
    if ultra > 0.0 && raid == 0.0 {
        let center = vec2(width * 0.5, height * 0.4);
        let max_radius = width.max(height) * 0.6;

        for i in 0..5 {
            let expansion_speed = 30.0;
            let radius =
                ((time * expansion_speed + i as f32 * max_radius / 5.0) % max_radius).max(1.0);

            let line_alpha = ultra * 0.25 * (1.0 - radius / max_radius);

            draw_circle_lines(
                center.x,
                center.y,
                radius,
                2.0 + ultra,
                Color::new(0.7, 0.3, 0.9, line_alpha),
            );
            draw_circle_lines(
                center.x,
                center.y,
                radius - 2.0,
                1.0,
                Color::new(1.0, 0.6, 0.9, line_alpha * 0.5),
            );
        }
    }

    // Raid mode: Trippy Kaleidoscopic Geometry
    if raid > 0.0 {
        let center = vec2(width * 0.5, height * 0.5);
        let max_radius = width.max(height) * 0.9;
        let num_shapes = 14 + (raid * 10.0) as i32;

        // Trippy screen tint pulsing with wild colors
        let r_flash = (time * 8.0).sin() * 0.5 + 0.5;
        let g_flash = (time * 11.0 + 2.0).sin() * 0.5 + 0.5;
        let b_flash = (time * 15.0 + 4.0).sin() * 0.5 + 0.5;
        draw_rectangle(
            0.0,
            0.0,
            width,
            height,
            Color::new(r_flash * 0.2, g_flash * 0.1, b_flash * 0.2, raid * 0.15),
        );

        // Counter-rotating, outward spiraling fractal polygons
        for i in 0..num_shapes {
            let phase = i as f32 / num_shapes as f32;

            // Shapes grow outwards and loop back smoothly
            let cycle = (time * 1.2 + phase).fract();
            let radius = max_radius * cycle.powf(1.5); // Exponential growth for depth illusion

            // Alternate rotation directions per shape layer
            let direction = if i % 2 == 0 { 1.0 } else { -1.0 };
            let rot = time * direction * (1.0 + cycle * 3.0);

            // Color shifting based on depth and time
            let r = (time * 3.0 + phase * std::f32::consts::PI * 2.0).sin() * 0.5 + 0.5;
            let g = (time * 4.0 + phase * std::f32::consts::PI * 2.0 + 2.0).sin() * 0.5 + 0.5;
            let b = (time * 5.0 + phase * std::f32::consts::PI * 2.0 + 4.0).sin() * 0.5 + 0.5;

            let alpha = raid * 0.6 * (1.0 - cycle); // Fade out as they get huge

            // Cycle between triangles, squares, and hexagons
            let sides = 3 + (i % 4) as u8;

            draw_poly_lines(
                center.x,
                center.y,
                sides,
                radius,
                rot * 180.0 / std::f32::consts::PI,
                2.0 + cycle * 8.0,
                Color::new(r, g, b, alpha),
            );
        }
    }
}

#[macroquad::main("Sword Dodge")]
async fn main() {
    let cultivator =
        Texture2D::from_file_with_format(include_bytes!("../assets/cultivator.png"), None);
    cultivator.set_filter(FilterMode::Nearest);
    let mut game = Game::new();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::LeftControl) {
            game.god_mode = !game.god_mode;
        }

        let dt = get_frame_time().min(0.05);

        if is_key_pressed(KeyCode::Space) && game.state == GameState::GameOver {
            game.reset();
        }

        game.update(dt);
        game.draw(&cultivator);

        next_frame().await;
    }
}
