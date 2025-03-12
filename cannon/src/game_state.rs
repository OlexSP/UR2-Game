use rusty_engine::prelude::*;

#[derive(Resource)]
pub  struct GameState {
    pub magnitude: (f32, bool),
    pub rotation: f32,
    pub ball_velocity: Vec2,
    pub score: u32,
    pub prev_mouse_x: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            magnitude: (15.0, false),
            rotation: 1.25,
            ball_velocity: Vec2::new(0.0, 0.0),
            score: 0,
            prev_mouse_x: 0.0,
        }
    }
}