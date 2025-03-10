use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    magnitude: f32,
    rotation: f32,
    ball_velocity: Vec2,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            magnitude: 0.0,
            rotation: 0.0,
            ball_velocity: Vec2::ZERO,
        }
    }
}
const GRAVITY_ACCELERATION: f32 = -9.81;
const BALL_LAYER: f32 = 1.0;
const CANON_LAYER: f32 = 2.0;

fn main() {
    let mut game = Game::new();
    let game_state = GameState::default();

    let ball = game.add_sprite("canon_ball", SpritePreset::RollingBallBlue);
    ball.collision = true;
    ball.translation = Vec2::new(0.0, -355.0);
    ball.layer = BALL_LAYER;

    let canon = game.add_sprite("canon", SpritePreset::RacingBarrierRed);
    canon.translation = Vec2::new(0.0, -355.0);
    canon.rotation = NORTH;
    canon.scale = 0.5;
    canon.layer = CANON_LAYER;




    game.add_logic(game_logic);
    game.run(game_state);


}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic goes here
}