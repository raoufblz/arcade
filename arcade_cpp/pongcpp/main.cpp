#include "raylib.h"
#include "raymath.h"
#include <cmath>

int main(){
    constexpr int 	screenWidth   = 1400;
    constexpr int 	screenHeight  = 800;
    constexpr float speed         = 400.0f;
    float 			speed_ball    = 500.0f;
    float 			max_speed     = 1500.0f;
    constexpr int 	rect_width    = 35;
    constexpr int 	rect_height   = 100;
    constexpr float ball_rad 	  = 30.0f;
    int 			score_left 	  = 0;
    int 			score_right   = 0;

    auto capSpeed = [&]() { if (speed_ball > max_speed) speed_ball = max_speed; };

    InitWindow(screenWidth, screenHeight, "pong!");
    SetTargetFPS(60);

    Vector2 position_1 = { 1350.0f - rect_width, 350.0f };
    Vector2 position_2 = { 50.0f, 350.0f };

    int angle = 0;
    while (angle % 90 == 0) angle = GetRandomValue(1,359);

    float 	radians    	   = angle * DEG2RAD;
    Vector2 direction_ball = { cosf(radians), sinf(radians) };
    Vector2 position_ball  = { 700.0f, 400.0f };
    SetExitKey(KEY_ESCAPE);

    while (!WindowShouldClose()){
        float delta 	   = GetFrameTime();
        float direction_1  = IsKeyDown(KEY_DOWN) - IsKeyDown(KEY_UP);
        float direction_2  = IsKeyDown(KEY_S) - IsKeyDown(KEY_W);

        position_1.y += direction_1 * speed * delta;

        //keeping rect inside window
        if (position_1.x < 0) 						 	position_1.x = 0;
        if (position_1.x + rect_width > screenWidth) 	position_1.x = screenWidth - rect_width;
        if (position_1.y < 0) 							position_1.y = 0;
        if (position_1.y + rect_height > screenHeight) 	position_1.y = screenHeight - rect_height;

        position_2.y += direction_2 * speed * delta;

        //keeping rect inside window
        if (position_2.x < 0) 							position_2.x = 0;
        if (position_2.x + rect_width > screenWidth) 	position_2.x = screenWidth - rect_width;
        if (position_2.y < 0) 							position_2.y = 0;
        if (position_2.y + rect_height > screenHeight) 	position_2.y = screenHeight - rect_height;

        if (speed_ball > max_speed) speed_ball = max_speed;
        direction_ball   = Vector2Normalize(direction_ball);
        position_ball.x += direction_ball.x * speed_ball * delta;
        position_ball.y += direction_ball.y * speed_ball * delta;

//==================== start of ball logic --------
        if (position_ball.x < ball_rad){
        position_ball.x = ball_rad;
        direction_ball.x *= -1;
        speed_ball *= 1.01f;
        score_right++;
        capSpeed();
        }

        if (position_ball.x + ball_rad > screenWidth){
        position_ball.x = screenWidth - ball_rad;
        direction_ball.x *= -1;
        speed_ball *= 1.01f;
        score_left++;
        capSpeed();
        }

        if (position_ball.y < ball_rad){
        position_ball.y = ball_rad;
        direction_ball.y *= -1;
        speed_ball *= 1.01f;
        capSpeed();
        }

        if (position_ball.y + ball_rad > screenHeight){
        position_ball.y = screenHeight - ball_rad;
        direction_ball.y *= -1;
        speed_ball *= 1.01f;
        capSpeed();
        }
//==================== end of ball logic (well i thought it was) --------
        BeginDrawing();
        ClearBackground(BLACK);

        DrawText(TextFormat("%d", score_right), screenWidth - 100, 50, 100, WHITE);
        DrawText(TextFormat("%d", score_left), 50, 50, 100, WHITE);

        Rectangle player_1 = { position_1.x, position_1.y, rect_width, rect_height };
        Rectangle player_2 = { position_2.x, position_2.y, rect_width, rect_height };
        DrawRectangleRec(player_1, (Color){255, 0, 0, 255});
        DrawRectangleRec(player_2, (Color){0, 0, 255, 255});

        Vector2 ball_center = { position_ball.x, position_ball.y };
        float ball_radius = ball_rad;
        DrawCircleV(ball_center, ball_radius, (Color){ 255, 255, 0, 255 });

        if (CheckCollisionCircleRec(ball_center, ball_radius, player_1)){
            direction_ball.x *= -1;
            speed_ball *= 1.01f;
            capSpeed();
            position_ball.x = player_1.x - ball_radius;
        }

        if (CheckCollisionCircleRec(ball_center, ball_radius, player_2)){
            direction_ball.x *= -1;
	        speed_ball *= 1.01f;
			capSpeed();
            position_ball.x = player_2.x + player_2.width + ball_radius;
        }

        EndDrawing();
    }

    CloseWindow();
    return 0;
}