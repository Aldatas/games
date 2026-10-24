#include "raylib.h"
#include <math.h>

// Shell Configs
#define SCREEN_WIDTH 400
#define SCREEN_HEIGHT 400
#define SCREEN_LABEL "Qi Collector"
#define TARGET_FPS 60

// Entity Configs
#define PLAYER_SPAWN {300, 225}
#define PLAYER_RADIUS 16
#define ORB_SPAWN {200, 150}
#define ORB_RADIUS 10
#define ORB_SPAWN_OFFSET 30

// Game Configs
#define SPEED 250.0f

int main(void) {
    InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, SCREEN_LABEL);
    SetTargetFPS(TARGET_FPS);

    Vector2 player = PLAYER_SPAWN;
    Vector2 orb = ORB_SPAWN;
    int score = 0;

    while (!WindowShouldClose()) {
        // Update
        float step = SPEED * GetFrameTime();
        Vector2 movement = {0.0f, 0.0f};

        if (IsKeyDown(KEY_W))
            movement.y -= step;
        if (IsKeyDown(KEY_S))
            movement.y += step;
        if (IsKeyDown(KEY_A))
            movement.x -= step;
        if (IsKeyDown(KEY_D))
            movement.x += step;

        float length = sqrtf(powf(movement.x, 2) + powf(movement.y, 2));
        if (length > step) {
            movement.x = (movement.x / length) * step;
            movement.y = (movement.y / length) * step;
        }

        player.x += movement.x;
        player.y += movement.y;

        if (player.x < PLAYER_RADIUS)
            player.x = PLAYER_RADIUS;

        if (player.x > SCREEN_WIDTH - PLAYER_RADIUS)
            player.x = SCREEN_WIDTH - PLAYER_RADIUS;

        if (player.y < PLAYER_RADIUS)
            player.y = PLAYER_RADIUS;

        if (player.y > SCREEN_HEIGHT - PLAYER_RADIUS)
            player.y = SCREEN_HEIGHT - PLAYER_RADIUS;

        if (CheckCollisionCircles(player, PLAYER_RADIUS, orb, ORB_RADIUS)) {
            score++;
            orb.x = GetRandomValue(ORB_SPAWN_OFFSET,
                                   SCREEN_WIDTH - ORB_SPAWN_OFFSET);
            orb.y = GetRandomValue(ORB_SPAWN_OFFSET,
                                   SCREEN_HEIGHT - ORB_SPAWN_OFFSET);
        }

        // DRAW
        BeginDrawing();
        ClearBackground(BLACK);

        DrawCircleV(player, PLAYER_RADIUS, VIOLET);
        DrawCircleV(orb, ORB_RADIUS, SKYBLUE);
        DrawText(TextFormat("Qi: %d", score), 20, 20, 24, WHITE);

        EndDrawing();
    }

    CloseWindow();
    return 0;
}
