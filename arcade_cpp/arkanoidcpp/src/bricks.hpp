#pragma once

#include "raylib.h"
#include "config.hpp"

class Brick {
public:
    explicit Brick(Vector2 position);

    bool isBroken() const { return broken; }
    void doBreak() { broken = true; }

    Rectangle rect() const;
    void draw(Color color) const;

    Vector2 position{};
    float width  = cfg::BRICK_WIDTH;
    float height = cfg::BRICK_HEIGHT;
    bool  broken = false;
};