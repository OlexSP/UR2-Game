use crate::constants::*;
use crate::game_state::GameState;
use rand::prelude::*;
use rand::rng;
use rusty_engine::prelude::*;

pub fn setup_sprites (game: &mut Game<GameState>, game_state: &GameState) {
    let wallpaper = game.add_sprite("wallpaper", "wallpapers/cannon_background.png");
    wallpaper.scale = 1.5 ;
    wallpaper.layer = WALL_LAYER;

    setup_cannon(game, game_state.rotation);
    setup_goal( game);
    setup_obstacles(game);
    setup_info_messages(game, game_state.magnitude, game_state.score, game_state.attempts);
    setup_slider(game, game_state.magnitude);
    setup_lives(game, &game_state.lives)
}
fn setup_cannon(game: &mut Game<GameState>, rotation: f32) {
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

fn setup_goal(game: &mut Game<GameState>) {
    let goal = game.add_sprite("goal", SpritePreset::RacingConeStraight);
    goal.translation = Vec2::new(
        rng().random_range(250.0..600.0),
        rng().random_range(-330.0..250.0),
    );
    goal.scale = 1.5;
    goal.layer = CANON_LAYER;
    goal.collision = true;
}

fn setup_obstacles(game: &mut Game<GameState>) {
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
        obstacle.layer = BALL_LAYER;
        obstacle.rotation = rng().random_range(RIGHT..LEFT);
        obstacle.collision = true;
    }
}

fn setup_info_messages(game: &mut Game<GameState>, magnitude: f32, score: u32, attempts: u32) {
    let magnitude_text = game.
        add_text("magnitude", format!("Magnitude: {:.1}", magnitude));
    magnitude_text.translation = Vec2::new(-530.0, 320.0);
    magnitude_text.layer = TEXT_LAYER;

    let score_text = game.add_text("score", format!(
        "Score: {}", score
    ));
    score_text.translation = Vec2::new(560.0, 320.0);
    score_text.layer = TEXT_LAYER;

    let attempts_text = game.add_text("attempts", format!(
        "Attempts: {}", attempts
    ));
    attempts_text.translation = Vec2::new(0.0, 320.0);
    attempts_text.layer = TEXT_LAYER;
}

fn setup_slider(game: &mut Game<GameState>, magnitude: f32) {
    let slider = game
        .add_sprite("magnitude_slider", SpritePreset::RacingBarrierRed);
    slider.translation = Vec2::new(-605.0 + magnitude * 6.0, 265.0);
    slider.scale = 0.15;
    slider.layer = TEXT_LAYER;
    slider.rotation = UP;

    let slider_bar = game
        .add_sprite("bar", SpritePreset::RollingBlockNarrow);
    slider_bar.translation = Vec2::new(-530.0, 265.0);
    slider_bar.layer = TEXT_LAYER - 1.0;
    slider_bar.scale = 1.5;
}

pub fn setup_lives(game: &mut Engine, lives: & u32) {
    for i in 0..*lives {
        let life_sprite = game
            .add_sprite(format!("life{}", i), SpritePreset::RacingCarRed);
        life_sprite.translation = Vec2::new(530.0 + i as f32 * 30.0, 270.0);
        life_sprite.scale = 0.3;
        life_sprite.rotation = UP;
        life_sprite.layer = TEXT_LAYER;
    }
}