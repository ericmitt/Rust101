# README

This Rust program simulates a flock of boids (bird-like objects) moving within a window, avoiding obstacles and each other while following the mouse cursor. The program uses the Piston window library for rendering and the `rand` crate for random number generation.

## Description

The program creates a window with dimensions 800x600 pixels and initializes 30 boids with random positions and velocities. It also generates 30 obstacles with random positions and sizes. The boids move towards the mouse cursor, avoiding obstacles and each other. When the mouse is moved, the boids update their positions to follow the cursor. If the mouse is clicked, the boids are randomized to new positions.

## How It Works

1. **Initialization**: The program initializes the window, boids, and obstacles. Boids are represented by the `Boid` struct, which contains their position and velocity. Obstacles are represented by the `Obstacle` struct, which contains their position and dimensions.

2. **Event Loop**: The main event loop listens for mouse movements and clicks. When the mouse is moved, the boids update their positions to follow the cursor. When the mouse is clicked, the boids are randomized to new positions.

3. **Boid Update**: The `update` method of the `Boid` struct calculates the direction to the target (mouse cursor) and updates the boid's velocity and position. Boids avoid obstacles and each other by adjusting their positions.

4. **Drawing**: The `draw` method of the `Boid` and `Obstacle` structs renders the boids and obstacles on the screen using the Piston window library.

This program demonstrates basic flocking behavior and obstacle avoidance using simple rules and randomization.

'''sh
cargo run max_angle=2