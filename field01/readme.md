# Particle Field Simulation

This project is a particle field simulation using the Piston game engine and the `rand` crate for random number generation. The simulation creates a field of particles that move, collide, and interact with each other within a window.

## Features

- Particles move with random directions and speeds.
- Particles bounce off the walls of the window.
- Particles collide with each other and transfer energy.
- Optional mouse interaction to apply impulses to particles.
- Configurable parameters for the simulation.

## Parameters

The simulation can be configured using command-line arguments. The following parameters are available:

- `width=<number>`: Width of the window (default: 800).
- `height=<number>`: Height of the window (default: 600).
- `circle_radius=<number>`: Radius of the circles (default: 10.0).
- `rows=<number>`: Number of rows of particles (default: 15).
- `cols=<number>`: Number of columns of particles (default: 20).
- `max_speed=<number>`: Maximum speed of particles (default: 5.0).
- `friction=<number>`: Friction coefficient (default: 0.01).
- `dev_angle=<number>`: Deviation angle during collisions (default: 0.02).
- `mouse_move=<true/false>`: Enable mouse move impulse (default: false).
- `trace_line=<true/false>`: Enable trace lines between particles (default: false).
- `transfer_rate=<number>`: Energy transfer rate during collisions (default: 0.5).
- `show_circle=<true/false>`: Show circles (default: true).
- `bezier=<true/false>`: Draw Bezier curves instead of lines (default: false).

## Usage

To run the simulation, use the following command:

```sh
cargo run help

cargo run max_speed=15 mouse_move=true friction=0.00001 circle_radius=15 transfer_rate=0.79 trace_line=true rows=5 cols=4 show_circle=false bezier=true

cargo run max_speed=15 mouse_move=true friction=0.00001 circle_radius=15 transfer_rate=0.79