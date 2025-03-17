# My Cannon Game

This is a simple 2D cannon game built using the Rust programming language and the `rusty_engine` crate.

## Scenario

The game was made according to the Ultimate Rust 2 course [scenario](https://github.com/CleanCut/rusty_engine/blob/main/scenarios/cannon_practice.md).


## Features

* **Adjustable Cannon:** Control the cannon's barrel angle and projectile magnitude.
* **Target Shooting:** Aim and shoot at a randomly placed target.
* **Obstacles:** Avoid randomly placed obstacles.
* **Score Tracking:** Keep track of your score.
* **Sound Effects and Music:** Enjoy background music and sound effects.
* **Keyboard and Mouse Controls:** Use keyboard or mouse to control the cannon.

## Prerequisites

* Rust programming language and Cargo (Rust's package manager) installed.

## How to Run

1.  **Clone the Repository:**
    ```
    git clone https://github.com/OlexSP/UR2-Game/cannon
    cd cannon
    ```

2.  **Run the Game:**
    ```
    cargo run --release
    ```

## Controls

* **Up/W:** Increase cannon barrel angle.
* **Down/S:** Decrease cannon barrel angle.
* **Left/A:** Decrease projectile magnitude.
* **Right/D:** Increase projectile magnitude.
* **Space/Middle Mouse Button:** Fire the cannon.

## Project Structure
```
cannon/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs         # Main entry point
    ├── game_state.rs   # Game state management
    ├── constants.rs    # Game constants
    ├── sprites.rs      # Sprite setup
    └── logic/          # Game logic
        ├── mod.rs
        ├── logic.rs
        ├── input.rs
        ├── projectile.rs
        └── collisions.rs    
```


## Dependencies

* `rusty_engine`: 2D game engine for Rust.
* `rand`: Random number generation.


## License

This project is licensed under the MIT License.