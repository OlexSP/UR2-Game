use rusty_engine::prelude::*;
use crate::constants::*;
use crate::game_state::GameState;

pub fn update_projectile(engine: &mut Engine, game_state: &mut GameState) {
    // Get cannon data once
    let cannon = engine.sprites.get_mut("cannon").unwrap();
    let c_translation = cannon.translation;
    let c_rotation = cannon.rotation;
    let _ = cannon; // Explicitly drop the mutable borrow early

    if let Some(canon_ball) = engine.sprites.get_mut("ball") {
        update_ball_physics(canon_ball, game_state, engine.delta_f32);
        // Check bounds and remove ball after physics update
        if canon_ball.translation.x > BALL_X_MAX
            || canon_ball.translation.y > BALL_Y_MAX
            || canon_ball.translation.y < BALL_Y_MIN
        {
            engine.sprites.remove("ball");
        }
    } else if engine.keyboard_state.just_pressed(KeyCode::Space)
    {
        spawn_ball(engine, game_state, c_translation, c_rotation);
    }
}

fn update_ball_physics(
    canon_ball: &mut Sprite,
    game_state: &mut GameState,
    delta: f32,
) {
    canon_ball.translation.x += game_state.ball_velocity.x * delta;
    canon_ball.translation.y += game_state.ball_velocity.y * delta;

    game_state.ball_velocity.y -= GRAVITY_ACCELERATION * delta;
    game_state.ball_velocity.y = match game_state.ball_velocity.y {
        y if y > 0.0 => y - AIR_RESISTANCE * delta,
        y if y < 0.0 => y + AIR_RESISTANCE * delta,
        y => y,
    };

    game_state.ball_velocity.x -= AIR_RESISTANCE * delta;
}

fn spawn_ball(
    engine: &mut Engine,
    game_state: &mut GameState,
    c_translation: Vec2,
    c_rotation: f32,
) {
    let cannon_ball = engine.add_sprite("ball", SpritePreset::RollingBallRed);
    cannon_ball.translation = c_translation;
    cannon_ball.rotation = c_rotation;
    cannon_ball.layer = BALL_LAYER;
    cannon_ball.collision = true;

    game_state.ball_velocity = Vec2::new(
        game_state.magnitude.0 * MAGNITUDE_MULTIPLIER * c_rotation.cos(),
        game_state.magnitude.0 * MAGNITUDE_MULTIPLIER * c_rotation.sin(),
    );

    engine.audio_manager.play_sfx(SfxPreset::Click, 0.2);
}