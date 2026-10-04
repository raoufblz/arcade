#include "ball.hpp"

#include <cmath>

Ball::Ball(Vector2 position) : position(position) {
    randomizeDirection();
}

void Ball::update(float delta) {
    position.x += direction.x * speed * delta;
    position.y += direction.y * speed * delta;
}

void Ball::reset() {
    position = Vector2{cfg::SCREEN_WIDTH / 2.0f, cfg::SCREEN_HEIGHT / 2.0f};
    speed = cfg::BALL_SPEED;
    randomizeDirection();
}

void Ball::capSpeed() {
    if (speed > cfg::MAX_SPEED) speed = cfg::MAX_SPEED;
}

void Ball::draw() const {
    DrawCircleV(position, radius, (Color){255, 255, 0, 255});
}

void Ball::randomizeDirection() {
    int angle = 0;
    while (angle % 90 == 0) {
        angle = GetRandomValue(15, 164);
    }

    const float radians = static_cast<float>(angle) * DEG2RAD;
    direction = Vector2{std::cos(radians), std::sin(radians)};
}