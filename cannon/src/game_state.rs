use rusty_engine::prelude::*;

#[derive(Resource)]
pub  struct GameState {
    pub lives: u32,
    pub attempts: u32,
    pub magnitude: (f32, bool),
    pub rotation: f32,
    pub ball_velocity: Vec2,
    pub score: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            lives: 3,
            attempts: 10,
            magnitude: (15.0, false),
            rotation: 1.25,
            ball_velocity: Vec2::new(0.0, 0.0),
            score: 0,
        }
    }
}