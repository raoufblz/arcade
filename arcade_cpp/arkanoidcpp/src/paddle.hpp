#pragma once

#include "raylib.h"
#include "config.hpp"

class Paddle {
public:
    explicit Paddle(Vector2 position);

    void update(int direction, float delta, float screenWidth);
    void reset(float x, float y);

    Rectangle rect() const;
    void draw(Color color) const;

    Vector2 position{};
    float width  = cfg::PADDLE_WIDTH;
    float height = cfg::PADDLE_HEIGHT;
    float speed  = cfg::PADDLE_SPEED;
};