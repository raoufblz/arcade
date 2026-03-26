#include "raylib.h"

int main(){
    const int screenWidth = 1400;
    const int screenHeight = 800;

    InitWindow(screenWidth, screenHeight, "pong!");
    SetTargetFPS(60);

    while (!WindowShouldClose()){

        BeginDrawing();
        SetExitKey(KEY_ESCAPE);
        ClearBackground(BLACK);
        EndDrawing();
    }

    CloseWindow();
    return 0;
}