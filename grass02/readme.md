# Grass Simulation

This project simulates the growth and reproduction of grass in a 2D grid world using Rust and the Piston game engine.

## Algorithm

1. **Initialization**: The world is initialized with a grid of size `WORLD_SIZE x WORLD_SIZE`, where each cell can either be empty or contain a grass object.
2. **Grass Growth**: Each grass object has an age that increases with each update.
3. **Reproduction**: Grass can reproduce if it reaches a certain age (`REPRODUCE_AGE`) and is not too old (`DEATH_AGE`). Reproduction occurs within a radius defined by `REPRO_RADIUS`.
4. **Death**: Grass dies if it reaches `DEATH_AGE`, which is randomized between the max and max ± 20% when a grass object is created.
5. **User Interaction**: Users can plant new grass by clicking on the grid.

## Parameters

- `WORLD_SIZE=<usize>`: Set the size of the world (default: 100).
- `CELL_SIZE=<f64>`: Set the size of each cell (default: 10.0).
- `MAX_SEEDS=<usize>`: Set the maximum number of seeds (default: 1).
- `REPRODUCE_AGE=<u32>`: Set the age at which grass can reproduce (default: 10).
- `DEATH_AGE=<u32>`: Set the age at which grass dies (default: 50).
- `REPRO_RADIUS=<isize>`: Set the reproduction radius (default: 3).
- `help`: Print the help message.

## Usage

1. **Run the Simulation**: Execute the program to start the simulation.
    ```sh
    cargo run -- [OPTIONS]
    ```
2. **Plant Grass**: Click on the grid to plant new grass at the clicked location.
3. **Observe**: Watch the grass grow, reproduce, and die over time.

## Example

```sh
cargo run -- WORLD_SIZE=150 CELL_SIZE=8.0 MAX_SEEDS=2 REPRODUCE_AGE=15 DEATH_AGE=60 REPRO_RADIUS=4
```

Click on the window to plant grass. The grass will grow, reproduce, and die based on the defined parameters.
