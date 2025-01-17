# README

This Rust program creates an interactive graphical application that draws circles and animates them along spiral trajectories. The program uses the Piston window library for rendering and the `rand` crate for random number generation.

## Description

The program opens a window with dimensions 800x600 pixels. When the user clicks within the window, a new circle is created at the mouse position and starts moving along a spiral trajectory. The circles can have random colors and their spiral direction can be either clockwise or counterclockwise. The program also supports connecting the centers of the circles with lines.

## How It Works

1. **Initialization**: The program initializes the window and sets up default values for various parameters such as circle radius, angle increment, spiral radius increment, and maximum angle for spiral rotation. These parameters can be overridden using command line arguments.

2. **Event Loop**: The main event loop listens for mouse movements and clicks. When the mouse is moved, the current mouse position is updated. When the left mouse button is clicked, a new circle is created at the mouse position if the number of circles is less than the specified limit.

3. **Circle Creation**: Each circle is initialized with a random color and an angle increment that determines the direction of the spiral (clockwise or counterclockwise). The circles are stored in a vector.

4. **Animation**: The circles are animated by updating their positions based on their angle and spiral radius. The angle and spiral radius are incremented in each frame to create the spiral effect. The maximum angle for the spiral rotation is enforced to ensure smooth trajectories.

5. **Drawing**: The circles are drawn on the screen using the Piston window library. If the `connect_line` parameter is set to `true`, lines are drawn between the centers of the circles.

## Command Line Parameters

- `num_circles=<number>`: Number of circles (default: 60)
- `radius1=<radius>`: Radius of circles (default: 10.0)
- `angle_increment=<increment>`: Angle increment for spiral (default: 0.05)
- `spiral_radius_increment=<increment>`: Spiral radius increment (default: 0.5)
- `max_angle=<angle>`: Maximum angle for spiral rotation (default: 30.0)
- `connect_line=<true/false>`: Connect circles with lines (default: false)

## Usage

To run the program with default parameters:

```sh
cargo run
```

To run the program with custom parameters:

```sh
cargo run -- num_circles=50 radius1=15.0 angle_increment=0.1 spiral_radius_increment=1.0 max_angle=45.0 connect_line=true
```

This will override the default values with the specified ones. The `help` parameter can be used to display the available command line options:

```sh
cargo run -- help
```

This program demonstrates basic animation and interaction using the Piston window library in Rust. It provides a fun and interactive way to visualize spiral trajectories and experiment with different parameters.