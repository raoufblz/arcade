#include "raylib.h"
// #include "raymath.h"

int main(){
    const int screenWidth = 1400;
    const int screenHeight = 800;
    const float speed = 300.0f;
    const int rect_width = 35;
    const int rect_height = 100;

    InitWindow(screenWidth, screenHeight, "pong!");
    SetTargetFPS(60);

    Vector2 position_1 = { 1350.0f - rect_width, 400.0f };
    Vector2 position_2 = {50.0f, 400.0f};

    while (!WindowShouldClose()){
        float delta = GetFrameTime();

        float move_player1 = IsKeyDown(KEY_DOWN) - IsKeyDown(KEY_UP);
        float move_player2 = IsKeyDown(KEY_S) - IsKeyDown(KEY_W);

        float direction_1 = move_player1;
        float direction_2 = move_player2;

        // normalize
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


        BeginDrawing();
        SetExitKey(KEY_ESCAPE);
        // HideCursor();
        ClearBackground(BLACK);

        DrawRectangle((int)position_1.x, (int)position_1.y, rect_width, rect_height, (Color){ 255, 0, 0, 255 });
        DrawRectangle((int)position_2.x, (int)position_2.y, rect_width, rect_height, (Color){ 0, 0, 255, 255 });
        DrawCircle((int)(1400.0f/2 - 15.0f), (int)(800.0f/2 - 15.0f), 30.0f, (Color){ 255, 255, 0, 255 });

        EndDrawing();
    }

    CloseWindow();
    return 0;
}