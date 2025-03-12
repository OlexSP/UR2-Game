use rand::prelude::*;
use rand::rng;
use rusty_engine::prelude::*;
use crate::constants::*;
use crate::game_state::GameState;

pub fn setup_cannon(game: &mut Game<GameState>, rotation: f32) {
    let cannon = game.add_sprite("cannon", SpritePreset::RacingBarrierRed);
    cannon.translation = Vec2::new(-550.0, -340.0);
    cannon.rotation = rotation;
    cannon.scale = 0.6;
    cannon.layer = CANON_LAYER;

    let cannon_wheel = game.add_sprite("wheel", SpritePreset::RollingBallBlue);
    cannon_wheel.translation = Vec2::new(-550.0, -345.0);
    cannon_wheel.scale = 1.0;
    cannon_wheel.layer = CANON_LAYER + 1.0;
}

pub fn setup_goal(game: &mut Game<GameState>) {
    let goal = game.add_sprite("goal", SpritePreset::RacingConeStraight);
    goal.translation = Vec2::new(
        rng().random_range(250.0..600.0),
        rng().random_range(-330.0..300.0),
    );
    goal.scale = 1.5;
    goal.layer = CANON_LAYER;
    goal.collision = true;
}

pub fn setup_obstacles(game: &mut Game<GameState>) {
    let obstacles = vec![
        SpritePreset::RollingBlockSquare,
        SpritePreset::RollingBlockCorner,
        SpritePreset::RollingBlockNarrow,
    ];

    for (i, preset) in obstacles.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.translation = Vec2::new(
            rng().random_range(-250.0..250.0),
            rng().random_range(-320.0..320.0),
        );
        obstacle.rotation = rng().random_range(RIGHT..LEFT);
        obstacle.collision = true;
    }
}

pub fn setup_text(game: &mut Game<GameState>, magnitude: f32, score: u32) {
    let magnitude_text = game.add_text("magnitude", format!("Magnitude: {}", magnitude));
    magnitude_text.translation = Vec2::new(-530.0, 320.0);
    magnitude_text.layer = TEXT_LAYER;

    let score_text = game.add_text("score", format!("Score {}", score));
    score_text.translation = Vec2::new(560.0, 320.0);
    score_text.layer = TEXT_LAYER;
}