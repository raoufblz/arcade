#include "raylib.h"
#include "raymath.h"
#include <cmath>

int main(){
    constexpr int 	screenWidth   = 1600;
    constexpr int 	screenHeight  = 900;
    constexpr float speed         = 400.0f;
    float 			speed_ball    = 500.0f;
    constexpr float max_speed     = 1500.0f;
    constexpr int 	rect_width    = 35;
    constexpr int 	rect_height   = 100;
    constexpr float ball_rad 	  = 30.0f;
    int 			score_right   = 0;
    int 			score_left    = 0;
    constexpr float paddle_offset = 50.0f;

    auto capSpeed = [&]() { if (speed_ball > max_speed) speed_ball = max_speed; };

    InitWindow(screenWidth, screenHeight, "pong!");
    SetTargetFPS(90);

    Vector2 position_pdl_right = { screenWidth - paddle_offset - rect_width, (screenHeight - rect_height) / 2.0f };
    Vector2 position_pdl_left = { paddle_offset, (screenHeight - rect_height) / 2.0f };

    int angle;
    do { angle = GetRandomValue(1, 359); }
    while (angle % 90 == 0 || (angle > 75 && angle < 105) || (angle > 255 && angle < 285));


    float 	radians    	   = angle * DEG2RAD;
    Vector2 direction_ball = { cosf(radians), sinf(radians) };
    Vector2 position_ball  = { screenWidth / 2.0f, screenHeight / 2.0f };
    SetExitKey(KEY_ESCAPE);

    auto reset_ball = [&]() {
        position_ball = { screenWidth / 2.0f, screenHeight / 2.0f };
        speed_ball = 500.0f;

        int angle;
        do { angle = GetRandomValue(1, 359); }
        while (angle % 90 == 0 || (angle > 75 && angle < 105) || (angle > 255 && angle < 285));

        float radians = angle * DEG2RAD;
        direction_ball = { cosf(radians), sinf(radians) };
        // Ensure ball goes toward a random player
        if (GetRandomValue(0, 1)) direction_ball.x *= -1;
    };


    while (!WindowShouldClose()){
        float delta 	   = GetFrameTime();
        float direction_pdl_right  = IsKeyDown(KEY_DOWN) - IsKeyDown(KEY_UP);
        float direction_pdl_left  = IsKeyDown(KEY_S) - IsKeyDown(KEY_W);

        position_pdl_right.y += direction_pdl_right * speed * delta;

        //keeping rect inside window
        if (position_pdl_right.y < 0) 							position_pdl_right.y = 0;
        if (position_pdl_right.y + rect_height > screenHeight) 	position_pdl_right.y = screenHeight - rect_height;

        position_pdl_left.y += direction_pdl_left * speed * delta;

        //keeping rect inside window
        if (position_pdl_left.y < 0) 							position_pdl_left.y = 0;
        if (position_pdl_left.y + rect_height > screenHeight) 	position_pdl_left.y = screenHeight - rect_height;

        capSpeed();
        // direction_ball   = Vector2Normalize(direction_ball);  // i think i ll need this
        position_ball.x += direction_ball.x * speed_ball * delta;
        position_ball.y += direction_ball.y * speed_ball * delta;

//==================== start of ball logic --------
        if (position_ball.x < ball_rad){
        	score_right++;
        	reset_ball();
        }

        if (position_ball.x + ball_rad > screenWidth){
        	score_left++;
        	reset_ball();
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
        Rectangle player_right = { position_pdl_right.x, position_pdl_right.y, rect_width, rect_height };
        Rectangle player_left = { position_pdl_left.x, position_pdl_left.y, rect_width, rect_height };

        Vector2 ball_center = { position_ball.x, position_ball.y };
        float ball_radius = ball_rad;

        if (CheckCollisionCircleRec(ball_center, ball_radius, player_right)){
            direction_ball.x *= -1;
            speed_ball *= 1.01f;
            capSpeed();
            position_ball.x = player_right.x - ball_radius;
        }

        if (CheckCollisionCircleRec(ball_center, ball_radius, player_left)){
            direction_ball.x *= -1;
	        speed_ball *= 1.01f;
			capSpeed();
            position_ball.x = player_left.x + player_left.width + ball_radius;
        }

        BeginDrawing();
        ClearBackground(BLACK);
		DrawFPS(10, 10);

        DrawText(TextFormat("%d", score_right), screenWidth - 100, 50, 100, WHITE);
        DrawText(TextFormat("%d", score_left), 50, 50, 100, WHITE);

        DrawRectangleRec(player_right, (Color){255, 0, 0, 255});
        DrawRectangleRec(player_left, (Color){0, 0, 255, 255});

        DrawCircleV(ball_center, ball_radius, (Color){ 255, 255, 0, 255 });

        EndDrawing();
    }

    CloseWindow();
    return 0;
}