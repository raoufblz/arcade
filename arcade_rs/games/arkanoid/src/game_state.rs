#[derive(PartialEq, Copy, Clone)]
pub enum GameState {
    Playing,
    Paused,
    GameOver,
    Win,
    Countdown(f32),
}
