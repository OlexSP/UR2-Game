use rusty_engine::prelude::*;

#[derive(Resource)]
pub  struct GameState {
    pub magnitude: f32,
    pub rotation: f32,
    pub ball_velocity: Vec2,
    pub score: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            magnitude: 15.0,
            rotation: 1.25,
            ball_velocity: Vec2::new(0.0, 0.0),
            score: 0,
        }
    }
}