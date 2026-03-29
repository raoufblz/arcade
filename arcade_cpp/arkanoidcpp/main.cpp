#include "raylib.h"

int main(){
    constexpr int	screenWidth  = 1400;
    constexpr int 	screenHeight = 800;
    constexpr float speed_paddle = 400.0f;
    float 			speed_ball   = 500.0f;
    constexpr int 	rect_width   = 100;
    constexpr int 	rect_height  = 35;
    constexpr float ball_rad 	 = 30.0f;
    int 			score 	 	 = 0;

    InitWindow(screenWidth, screenHeight, "arkanoid!");
    SetTargetFPS(60);
    SetExitKey(KEY_ESCAPE);

    Vector2 paddle = { (screenWidth - rect_width) /2.0f , 700.0f };

    while (!WindowShouldClose()){
	    float delta 	   = GetFrameTime();
	    float direction_1  = (IsKeyDown(KEY_RIGHT) || IsKeyDown(KEY_D)) - (IsKeyDown(KEY_LEFT) || IsKeyDown(KEY_A));
	    paddle.x += direction_1 * speed_paddle * delta;

        if (paddle.x < 0) 						 	paddle.x = 0;
        if (paddle.x + rect_width > screenWidth) 	paddle.x = screenWidth - rect_width;

        BeginDrawing();
        ClearBackground(BLACK);

        Rectangle paddle_rect = { paddle.x, paddle.y, rect_width, rect_height };
        DrawRectangleRec(paddle_rect, (Color){255, 0, 0, 255});

        EndDrawing();
    }

    CloseWindow();
    return 0;
}