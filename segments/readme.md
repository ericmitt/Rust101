# Rust Piston Window Example

This project demonstrates how to create a graphical application using the Piston window library in Rust. The application generates random points within a window and connects them with lines. It also allows for customization through command line arguments.

## Features

- Generate random internal points that move within the window.
- Generate edge points that stay on the edges of the window.
- Draw lines between points with optional colorization.
- Resize the window dynamically.
- Customize various parameters through command line arguments.

## Command Line Arguments

You can customize the behavior of the application using the following command line arguments:

- `width=<value>`: Set the window width (default: 800).
- `height=<value>`: Set the window height (default: 600).
- `colorized=<true|false>`: Enable or disable colorized zones (default: true).
- `num_internal_points=<value>`: Set the number of internal points (default: 8).
- `num_edge_points=<value>`: Set the number of edge points (default: 5).
- `point_radius=<value>`: Set the radius of the points (default: 5.0).
- `invisible_lines=<true|false>`: Set lines to be invisible (default: false).

## Usage

To run the application with default settings, simply execute:

```sh
cargo run
```

To customize the settings, you can pass command line arguments. For example:

```sh
cargo run -- width=1024 height=768 colorized=false num_internal_points=10 num_edge_points=8 point_radius=3.0 invisible_lines=true
cargo run colorized=true num_edge_points=10 invisible_lines=true num_internal_points=20
cargo run -- width=1024 height=768 num_internal_points=10 num_edge_points=8 invisible_lines=true
```

