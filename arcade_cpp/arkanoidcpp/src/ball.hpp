#pragma once

#include "raylib.h"
#include "config.hpp"

class Ball {
public:
    explicit Ball(Vector2 position);

    void update(float delta);
    void reset();
    void capSpeed();
    void draw() const;

    Vector2 position{};
    Vector2 direction{};
    float speed  = cfg::BALL_SPEED;
    float radius = cfg::BALL_RADIUS;

private:
    void randomizeDirection();
};