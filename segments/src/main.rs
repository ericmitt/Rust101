extern crate piston_window;

use piston_window::*;
use rand::Rng;
use std::env;
use std::f64;
use std::thread;
use std::time::Duration;

const WINDOW_WIDTH: f64 = 800.0;
const WINDOW_HEIGHT: f64 = 600.0;
const NUM_INTERNAL_POINTS: usize = 8;
const NUM_EDGE_POINTS: usize = 5;
const POINT_RADIUS: f64 = 5.0;
const LINE_WIDTH: f64 = 1.0;
const SPEED_MIN: f64 = 1.0;
const SPEED_MAX: f64 = 3.0;
const DX_MIN: f64 = -1.0;
const DX_MAX: f64 = 1.0;
const DY_MIN: f64 = -1.0;
const DY_MAX: f64 = 1.0;
const _TIME_SCALE: f64 = 0.5;
const COLORIZED: bool = true;
const INVISIBLE_LINES: bool = false;

#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    speed: f64,
}

impl Point {
    fn new(x: f64, y: f64, dx: f64, dy: f64, speed: f64) -> Self {
        Point {
            x,
            y,
            dx,
            dy,
            speed,
        }
    }

    fn update(&mut self, width: f64, height: f64) {
        self.x += self.dx * self.speed;
        self.y += self.dy * self.speed;

        if self.x <= 0.0 || self.x >= width {
            self.dx = -self.dx;
        }
        if self.y <= 0.0 || self.y >= height {
            self.dy = -self.dy;
        }
    }
}

fn generate_random_points(num_points: usize, width: f64, height: f64) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let mut points = Vec::new();

    for _ in 0..num_points {
        let x = rng.gen_range(0.0..width);
        let y = rng.gen_range(0.0..height);
        let dx = rng.gen_range(DX_MIN..DX_MAX);
        let dy = rng.gen_range(DY_MIN..DY_MAX);
        let speed = rng.gen_range(SPEED_MIN..SPEED_MAX);
        points.push(Point::new(x, y, dx, dy, speed));
    }

    points
}

fn generate_edge_points(num_points: usize, width: f64, height: f64) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let mut points = Vec::new();

    for _ in 0..num_points {
        points.push(Point::new(
            rng.gen_range(1.0..(width - 1.0)),
            1.0,
            0.0,
            0.0,
            0.0,
        )); // Top edge
        points.push(Point::new(
            rng.gen_range(1.0..(width - 1.0)),
            height - 1.0,
            0.0,
            0.0,
            0.0,
        )); // Bottom edge
        points.push(Point::new(
            1.0,
            rng.gen_range(1.0..(height - 1.0)),
            0.0,
            0.0,
            0.0,
        )); // Left edge
        points.push(Point::new(
            width - 1.0,
            rng.gen_range(1.0..(height - 1.0)),
            0.0,
            0.0,
            0.0,
        )); // Right edge
    }

    points
}

fn print_help() {
    println!("Usage: program [options]");
    println!("Options:");
    println!("  width=<value>               Set the window width (default: 800)");
    println!("  height=<value>              Set the window height (default: 600)");
    println!("  colorized=<true|false>      Enable or disable colorized zones (default: true)");
    println!("  num_internal_points=<value> Set the number of internal points (default: 8)");
    println!("  num_edge_points=<value>     Set the number of edge points (default: 5)");
    println!("  point_radius=<value>        Set the radius of the points (default: 5.0)");
    println!("  invisible_lines=<true|false> Set lines to be invisible (default: false)");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "help" || arg == "h") {
        print_help();
        return;
    }

    let mut width = args
        .iter()
        .find(|arg| arg.starts_with("width="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(WINDOW_WIDTH);

    let mut height = args
        .iter()
        .find(|arg| arg.starts_with("height="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(WINDOW_HEIGHT);

    let colorized = args
        .iter()
        .find(|arg| arg.starts_with("colorized="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(COLORIZED);

    let num_internal_points = args
        .iter()
        .find(|arg| arg.starts_with("num_internal_points="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(NUM_INTERNAL_POINTS);

    let num_edge_points = args
        .iter()
        .find(|arg| arg.starts_with("num_edge_points="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(NUM_EDGE_POINTS);

    let point_radius = args
        .iter()
        .find(|arg| arg.starts_with("point_radius="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(POINT_RADIUS);

    let invisible_lines = args
        .iter()
        .find(|arg| arg.starts_with("invisible_lines="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(INVISIBLE_LINES);

    let mut window: PistonWindow =
        WindowSettings::new("Resizable Window", [width as u32, height as u32])
            .resizable(true)
            .build()
            .unwrap();

    let mut internal_points = generate_random_points(num_internal_points, width, height);
    let mut external_points = generate_edge_points(num_edge_points, width, height);
    let mut all_points = internal_points.clone();
    //all_points.extend(external_points.clone());
    //let mut time = 0.0;

    let mut region_colors =
        vec![[0.0, 0.0, 0.0, 0.0]; all_points.len() * all_points.len() * all_points.len()];

    while let Some(event) = window.next() {
        if let Some(_args) = event.render_args() {
            window.draw_2d(&event, |c, g, _| {
                clear([1.0; 4], g);

                if colorized {
                    let mut rng = rand::thread_rng();
                    for i in 0..all_points.len() {
                        for j in i + 1..all_points.len() {
                            for k in j + 1..all_points.len() {
                                let index = i * all_points.len() * all_points.len()
                                    + j * all_points.len()
                                    + k;
                                if region_colors[index] == [0.0, 0.0, 0.0, 0.0] {
                                    region_colors[index] = [rng.gen(), rng.gen(), rng.gen(), 1.0];
                                }
                                polygon(
                                    region_colors[index],
                                    &[
                                        [all_points[i].x, all_points[i].y],
                                        [all_points[j].x, all_points[j].y],
                                        [all_points[k].x, all_points[k].y],
                                    ],
                                    c.transform,
                                    g,
                                );
                            }
                        }
                    }
                }

                for point in &all_points {
                    ellipse(
                        [1.0, 0.0, 0.0, 1.0],
                        [point.x, point.y, point_radius, point_radius],
                        c.transform,
                        g,
                    );
                }

                let line_color = if invisible_lines {
                    [0.0, 0.0, 0.0, 0.0]
                } else {
                    [0.0, 0.0, 0.0, 1.0]
                };

                for i in 0..internal_points.len() {
                    for j in i + 1..internal_points.len() {
                        line_from_to(
                            line_color,
                            LINE_WIDTH,
                            [internal_points[i].x, internal_points[i].y],
                            [internal_points[j].x, internal_points[j].y],
                            c.transform,
                            g,
                        );
                    }
                }

                for external_point in &external_points {
                    let mut closest_point = &internal_points[0];
                    let mut min_distance = f64::MAX;

                    for internal_point in &internal_points {
                        let distance = ((external_point.x - internal_point.x).powi(2)
                            + (external_point.y - internal_point.y).powi(2))
                        .sqrt();

                        if distance < min_distance {
                            min_distance = distance;
                            closest_point = internal_point;
                        }
                    }

                    line_from_to(
                        line_color,
                        LINE_WIDTH,
                        [external_point.x, external_point.y],
                        [closest_point.x, closest_point.y],
                        c.transform,
                        g,
                    );
                }

                if external_points.len() > 1 {
                    for i in 0..external_points.len() {
                        let next_index = (i + 1) % external_points.len();
                        line_from_to(
                            line_color,
                            LINE_WIDTH,
                            [external_points[i].x, external_points[i].y],
                            [external_points[next_index].x, external_points[next_index].y],
                            c.transform,
                            g,
                        );
                    }
                }

                // Draw lines between all external points
                for i in 0..external_points.len() {
                    for j in i + 1..external_points.len() {
                        line_from_to(
                            line_color,
                            LINE_WIDTH,
                            [external_points[i].x, external_points[i].y],
                            [external_points[j].x, external_points[j].y],
                            c.transform,
                            g,
                        );
                    }
                }
            });
        }

        if let Some(_args) = event.update_args() {
            //time += args.dt;
            for point in &mut internal_points {
                point.update(width, height);
            }
            all_points = internal_points.clone();
            // all_points.extend(external_points.clone());
            thread::sleep(Duration::from_millis(30));
        }

        if let Some(args) = event.resize_args() {
            let new_width = args.window_size[0] as f64;
            let new_height = args.window_size[1] as f64;
            internal_points = generate_random_points(num_internal_points, new_width, new_height);
            external_points = generate_edge_points(num_edge_points, new_width, new_height);
            all_points = internal_points.clone();
            // all_points.extend(external_points.clone());
            width = new_width;
            height = new_height;
        }
    }
}
