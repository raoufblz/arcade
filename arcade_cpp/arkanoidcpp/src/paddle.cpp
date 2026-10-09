#include "paddle.hpp"

#include <algorithm>

Paddle::Paddle(Vector2 position) : position(position) {}

void Paddle::update(int direction, float delta, float screenWidth) {
    position.x += static_cast<float>(direction) * speed * delta;
    position.x = std::clamp(position.x, 0.0f, screenWidth - width);
}

void Paddle::reset(float x, float y) {
    position = Vector2{x, y};
}

Rectangle Paddle::rect() const {
    return Rectangle{position.x, position.y, width, height};
}

void Paddle::draw(Color color) const {
    DrawRectangleRec(rect(), color);
}