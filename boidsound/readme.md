# Boids Simulation with Sound-Controlled Target

This program simulates a flock of boids (bird-like objects) that move around the screen, avoiding obstacles and following a target. The target's position is controlled by the average frequency of sound input, allowing for interactive control of the boids' behavior.

## Features

- Simulates a flock of boids with realistic movement and obstacle avoidance.
- Target position is controlled by sound frequency input.
- Customizable parameters for the number of boids, boid speed, number of obstacles, maximum angle for boid rotation, and frequency thresholds.

## Usage

To run the program, use the following command:

```sh
cargo run help
cargo run num_obs=5 num_boids=10 boid_speed=1 freq_thresholds=0.00190,0.00230,0.00500 max_angle=5.7
cargo run num_boids=5 num_obs=3 max_angle=2