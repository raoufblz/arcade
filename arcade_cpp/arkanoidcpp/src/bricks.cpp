#include "bricks.hpp"

Brick::Brick(Vector2 position) : position(position) {}

Rectangle Brick::rect() const {
    return Rectangle{position.x, position.y, width, height};
}

void Brick::draw(Color color) const {
    DrawRectangleRec(rect(), color);
}