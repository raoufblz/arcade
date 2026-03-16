#include "raylib.h"
#include "raymath.h"
#include <cmath>

int main(){
    const int 	screenWidth = 	1400;
    const int 	screenHeight = 	800;
    const float speed = 		300.0f;
    const int 	rect_width = 	35;
    const int 	rect_height = 	100;
    const float ball_rad = 		30.0f;

    InitWindow(screenWidth, screenHeight, "pong!");
    SetTargetFPS(60);

    Vector2 position_1 = { 1350.0f - rect_width, 350.0f };
    Vector2 position_2 = {50.0f, 350.0f};

    int ballpos_y = GetRandomValue(15,785);
    int angle = GetRandomValue(1, 359);

    Vector2 position_ball = {700.0f, 400.0f};
    while (!WindowShouldClose()){
        float delta = GetFrameTime();

        float direction_1 = IsKeyDown(KEY_DOWN) - IsKeyDown(KEY_UP);
        float direction_2 = IsKeyDown(KEY_S) - IsKeyDown(KEY_W);

        // normalize --> for the ball now
        //direction = Vector2Normalize(direction);

        position_1.x += 0;
        position_1.y += direction_1 * speed * delta;

        //keeping rect inside window, needs fixing as players are locked
        if (position_1.x < 0) position_1.x = 0;
        if (position_1.x + rect_width > screenWidth) position_1.x = screenWidth - rect_width;
        if (position_1.y < 0) position_1.y = 0;
        if (position_1.y + rect_height > screenHeight) position_1.y = screenHeight - rect_height;


        position_2.x += 0;
        position_2.y += direction_2 * speed * delta;

        //keeping rect inside window
        if (position_2.x < 0) position_2.x = 0;
        if (position_2.x + rect_width > screenWidth) position_2.x = screenWidth - rect_width;
        if (position_2.y < 0) position_2.y = 0;
        if (position_2.y + rect_height > screenHeight) position_2.y = screenHeight - rect_height;


        float radians = angle * DEG2RAD;
        Vector2 direction_ball = { cos(radians), sin(radians) };
        direction_ball = Vector2Normalize(direction_ball);
        position_ball.x += direction_ball.x * speed * delta;
        position_ball.y += direction_ball.y * speed * delta;

        //keeping ball inside window
        if (position_ball.x < ball_rad) position_ball.x = ball_rad;
        if (position_ball.x + ball_rad > screenWidth) position_ball.x = screenWidth - ball_rad;
        if (position_ball.y < ball_rad) position_ball.y = ball_rad;
        if (position_ball.y + ball_rad > screenHeight) position_ball.y = screenHeight - ball_rad;

        BeginDrawing();
        SetExitKey(KEY_ESCAPE);
        ClearBackground(BLACK);

        DrawRectangle((int)position_1.x, (int)position_1.y, rect_width, rect_height, (Color){ 255, 0, 0, 255 });
        DrawRectangle((int)position_2.x, (int)position_2.y, rect_width, rect_height, (Color){ 0, 0, 255, 255 });
        DrawCircle((int)position_ball.x, (int)position_ball.y, ball_rad, (Color){ 255, 255, 0, 255 });

        EndDrawing();
    }

    CloseWindow();
    return 0;
}