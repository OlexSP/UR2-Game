#![allow(dead_code, unused_variables)]

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
            magnitude: 15.0,
            rotation: 1.25,
            ball_velocity: Vec2::new(0.0, 0.0),
        }
    }
}
const GRAVITY_ACCELERATION: f32 = 100.0;
const AIR_RESISTANCE: f32 = 15.0;
const MAGNITUDE_MULTIPLIER: f32 = 20.0;
const MAGNITUDE_CHANGING_SPEED: f32 = 2.0;
const ROTATION_SPEED: f32 = 0.5;
const BALL_LAYER: f32 = 1.0;
const CANON_LAYER: f32 = 2.0;
const TEXT_LAYER: f32 = 3.0;

fn main() {
    let mut game = Game::new();
    let game_state = GameState::default();

    let cannon = game.add_sprite("cannon", SpritePreset::RacingBarrierRed);
    cannon.translation = Vec2::new(-550.0, -340.0);
    cannon.rotation = game_state.rotation;
    cannon.scale = 0.6;
    cannon.layer = CANON_LAYER;

    let cannon_wheel = game.add_sprite("wheel", SpritePreset::RollingBallBlue);
    cannon_wheel.translation = Vec2::new(-550.0, -345.0);
    cannon_wheel.scale = 1.0;
    cannon_wheel.layer = CANON_LAYER + 1.0;

    let goal = game.add_sprite("goal", SpritePreset::RacingConeStraight);
    goal.translation = Vec2::new(400.0, -320.0);
    goal.scale = 1.5;
    goal.layer = CANON_LAYER;

    let obstacles = vec![
        SpritePreset::RollingBlockSquare,
        SpritePreset::RollingBlockCorner,
        SpritePreset::RollingBlockNarrow,
    ];

    for (i, preset) in obstacles.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.translation = Vec2::new(-100.0 + i as f32 * 100.0, -300.0);
        obstacle.collision = true;
    }

    // text
    let magnitude_text = game.add_text("magnitude", format!("Magnitude: {}", game_state.magnitude));
    magnitude_text.translation = Vec2::new(-540.0, 320.0);
    magnitude_text.layer = TEXT_LAYER;

    // music
    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // keyboard input
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        game_state.rotation += ROTATION_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        game_state.rotation -= ROTATION_SPEED * engine.delta_f32;
    }
    game_state.rotation = game_state.rotation.clamp(RIGHT, UP);

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A])
    {
        game_state.magnitude -= MAGNITUDE_CHANGING_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::D])
    {
        game_state.magnitude += MAGNITUDE_CHANGING_SPEED * engine.delta_f32;
    }
    game_state.magnitude = game_state.magnitude.clamp(0.0, 25.0);

    // text messages
    let magnitude_message = engine.texts.get_mut("magnitude").unwrap();
    magnitude_message.value = format!("Magnitude: {:.1}", game_state.magnitude);

    // cannon rotation movement
    let cannon = engine.sprites.get_mut("cannon").unwrap();
    cannon.rotation = game_state.rotation;
    let c_translation = cannon.translation;
    let c_rotation = cannon.rotation;

    // canon bal movement
    if let Some(canon_ball) = engine.sprites.get_mut("ball") {
        canon_ball.translation.x += game_state.ball_velocity.x * engine.delta_f32;
        canon_ball.translation.y += game_state.ball_velocity.y * engine.delta_f32;

        game_state.ball_velocity.y -= GRAVITY_ACCELERATION * engine.delta_f32;
        game_state.ball_velocity.y = match game_state.ball_velocity.y {
            y if y > 0.0 => game_state.ball_velocity.y - AIR_RESISTANCE * engine.delta_f32,
            y if y < 0.0 => game_state.ball_velocity.y + AIR_RESISTANCE * engine.delta_f32,
            _ => game_state.ball_velocity.y,
        };

        game_state.ball_velocity.x -= AIR_RESISTANCE * engine.delta_f32;
        game_state.ball_velocity.x = game_state.ball_velocity.x.clamp(0.0, 1000.0);

        if canon_ball.translation.x > 750.0
            || canon_ball.translation.y > 400.0
            || canon_ball.translation.y < -400.0
        {
            engine.sprites.remove("ball");
        }
    } else if engine.keyboard_state.just_pressed(KeyCode::Space)
        || engine.mouse_state.just_pressed(MouseButton::Left)
    {
        let cannon_ball = engine.add_sprite("ball", SpritePreset::RollingBallRed);
        cannon_ball.translation = c_translation;
        cannon_ball.rotation = c_rotation;
        cannon_ball.layer = BALL_LAYER;

        game_state.ball_velocity = Vec2::new(
            game_state.magnitude * MAGNITUDE_MULTIPLIER * c_rotation.cos(),
            game_state.magnitude * MAGNITUDE_MULTIPLIER * c_rotation.sin(),
        );

        engine.audio_manager.play_sfx(SfxPreset::Click, 0.2);
    }
}
