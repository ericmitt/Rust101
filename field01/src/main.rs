extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;
use std::env;
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;
const DEFAULT_CIRCLE_RADIUS: f64 = 10.0;
const DEFAULT_ROWS: usize = 15;
const DEFAULT_COLS: usize = 20;
const DEFAULT_MAX_SPEED: f64 = 5.0;
const DEFAULT_FRICTION: f64 = 0.01;
const DEFAULT_DEV_ANGLE: f64 = 0.02;
const DEFAULT_MOUSE_MOVE: bool = false;
const DEFAULT_TRACE_LINE: bool = false;
const DEFAULT_TRANSFER_RATE: f64 = 0.5;
const DEFAULT_SHOW_CIRCLE: bool = true;
const DEFAULT_BEZIER: bool = false;

#[derive(Clone, Copy)]
struct Particle {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    speed: f64,
    color: [f32; 4],
}

impl Particle {
    fn new(x: f64, y: f64, color: [f32; 4]) -> Self {
        Particle {
            x,
            y,
            dx: 0.0,
            dy: 0.0,
            speed: 0.0,
            color,
        }
    }

    fn update(&mut self, friction: f64, width: f64, height: f64, circle_radius: f64) {
        self.x += self.dx * self.speed;
        self.y += self.dy * self.speed;

        // Apply friction
        self.speed *= 1.0 - friction;

        // Bounce off walls
        if self.x < circle_radius || self.x > width - circle_radius {
            self.dx = -self.dx;
        }
        if self.y < circle_radius || self.y > height - circle_radius {
            self.dy = -self.dy;
        }
    }

    fn set_random_direction_and_speed(&mut self, max_speed: f64) {
        let mut rng = rand::rng();
        let angle = rng.random_range(0.0..2.0 * PI);
        self.dx = angle.cos();
        self.dy = angle.sin();
        self.speed = rng.random_range(0.0..max_speed);
    }

    fn apply_impulse(&mut self, impulse: f64) {
        let mut rng = rand::rng();
        let angle = rng.random_range(0.0..2.0 * PI);
        self.dx += angle.cos() * impulse;
        self.dy += angle.sin() * impulse;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("help")) {
        println!("Usage: cargo run -- [OPTIONS]");
        println!("Options:");
        println!("  width=<number>              Width of the window (default: 800)");
        println!("  height=<number>             Height of the window (default: 600)");
        println!("  circle_radius=<number>      Radius of the circles (default: 10.0)");
        println!("  rows=<number>               Number of rows (default: 15)");
        println!("  cols=<number>               Number of columns (default: 20)");
        println!("  max_speed=<number>          Maximum speed of particles (default: 5.0)");
        println!("  friction=<number>           Friction coefficient (default: 0.01)");
        println!("  dev_angle=<number>          Deviation angle (default: 0.02)");
        println!("  mouse_move=<true/false>     Enable mouse move impulse (default: false)");
        println!("  trace_line=<true/false>     Enable trace lines (default: false)");
        println!(
            "  transfer_rate=<number>      Energy transfer rate during collision (default: 0.5)"
        );
        println!("  show_circle=<true/false>    Show circles (default: true)");
        println!(
            "  bezier=<true/false>         Draw Bezier curves instead of lines (default: false)"
        );
        return;
    }

    let width = args
        .iter()
        .find(|arg| arg.starts_with("width="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_WIDTH);

    let height = args
        .iter()
        .find(|arg| arg.starts_with("height="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_HEIGHT);

    let circle_radius = args
        .iter()
        .find(|arg| arg.starts_with("circle_radius="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_CIRCLE_RADIUS);

    let rows = args
        .iter()
        .find(|arg| arg.starts_with("rows="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_ROWS);

    let cols = args
        .iter()
        .find(|arg| arg.starts_with("cols="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_COLS);

    let max_speed = args
        .iter()
        .find(|arg| arg.starts_with("max_speed="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_MAX_SPEED);

    let friction = args
        .iter()
        .find(|arg| arg.starts_with("friction="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_FRICTION);

    let _dev_angle = args
        .iter()
        .find(|arg| arg.starts_with("dev_angle="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_DEV_ANGLE);

    let mouse_move = args
        .iter()
        .find(|arg| arg.starts_with("mouse_move="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_MOUSE_MOVE);

    let trace_line = args
        .iter()
        .find(|arg| arg.starts_with("trace_line="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_TRACE_LINE);

    let transfer_rate = args
        .iter()
        .find(|arg| arg.starts_with("transfer_rate="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_TRANSFER_RATE);

    let show_circle = args
        .iter()
        .find(|arg| arg.starts_with("show_circle="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_SHOW_CIRCLE);

    let bezier = args
        .iter()
        .find(|arg| arg.starts_with("bezier="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_BEZIER);

    let mut window: PistonWindow = WindowSettings::new("Particle Field", [width, height])
        .resizable(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut particles = vec![];
    for i in 0..rows {
        for j in 0..cols {
            let x = j as f64 * (width as f64 / cols as f64) + circle_radius;
            let y = i as f64 * (height as f64 / rows as f64) + circle_radius;
            let color = [rand::random(), rand::random(), rand::random(), 1.0];
            particles.push(Particle::new(x, y, color));
        }
    }

    let target = Arc::new(Mutex::new((0.0, 0.0)));
    let mut current_width = width as f64;
    let mut current_height = height as f64;

    while let Some(event) = window.next() {
        if let Some(mouse_pos) = event.mouse_cursor_args() {
            let mut target = target.lock().unwrap();
            *target = (mouse_pos[0], mouse_pos[1]);

            if mouse_move {
                for particle in &mut particles {
                    let dx = particle.x - target.0;
                    let dy = particle.y - target.1;
                    if dx * dx + dy * dy < circle_radius * circle_radius {
                        particle.apply_impulse(max_speed);
                    }
                }
            }
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            let target = target.lock().unwrap();
            for particle in &mut particles {
                let dx = particle.x - target.0;
                let dy = particle.y - target.1;
                if dx * dx + dy * dy < circle_radius * circle_radius {
                    particle.set_random_direction_and_speed(max_speed);
                }
            }
        }

        if let Some(Button::Keyboard(Key::R)) = event.press_args() {
            for i in 0..rows {
                for j in 0..cols {
                    let x = j as f64 * (current_width / cols as f64) + circle_radius;
                    let y = i as f64 * (current_height / rows as f64) + circle_radius;
                    particles[i * cols + j].x = x;
                    particles[i * cols + j].y = y;
                    particles[i * cols + j].dx = 0.0;
                    particles[i * cols + j].dy = 0.0;
                    particles[i * cols + j].speed = 0.0;
                }
            }
        }

        if let Some(args) = event.resize_args() {
            current_width = args.window_size[0] as f64;
            current_height = args.window_size[1] as f64;
            window.set_size([args.window_size[0] as u32, args.window_size[1] as u32]);
        }

        for i in 0..particles.len() {
            for j in (i + 1)..particles.len() {
                let dx = particles[i].x - particles[j].x;
                let dy = particles[i].y - particles[j].y;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance < 2.0 * circle_radius {
                    // Collision detected

                    // Calculate the normal and tangent vectors
                    let normal_x = dx / distance;
                    let normal_y = dy / distance;
                    let tangent_x = -normal_y;
                    let tangent_y = normal_x;

                    // Project the velocities onto the normal and tangent vectors
                    let v1n = normal_x * particles[i].dx + normal_y * particles[i].dy;
                    let v1t = tangent_x * particles[i].dx + tangent_y * particles[i].dy;
                    let v2n = normal_x * particles[j].dx + normal_y * particles[j].dy;
                    let v2t = tangent_x * particles[j].dx + tangent_y * particles[j].dy;

                    // Calculate the new normal velocities using transfert_speed
                    let new_v1n = transfer_rate * v2n + (1.0 - transfer_rate) * v1n;
                    let new_v2n = transfer_rate * v1n + (1.0 - transfer_rate) * v2n;

                    // Calculate the new velocities
                    particles[i].dx = new_v1n * normal_x + v1t * tangent_x;
                    particles[i].dy = new_v1n * normal_y + v1t * tangent_y;
                    particles[j].dx = new_v2n * normal_x + v2t * tangent_x;
                    particles[j].dy = new_v2n * normal_y + v2t * tangent_y;

                    // Ensure the particles start moving
                    particles[i].speed = (particles[i].dx.powi(2) + particles[i].dy.powi(2)).sqrt();
                    particles[j].speed = (particles[j].dx.powi(2) + particles[j].dy.powi(2)).sqrt();

                    // Debug statements
                    // println!("Collision detected between particles {} and {}", i, j);
                    // println!("Particle {} speed: {}", i, particles[i].speed);
                    // println!("Particle {} speed: {}", j, particles[j].speed);

                    // Move collided particles apart
                    let overlap = 2.0 * circle_radius - distance;
                    let move_x = overlap * dx / distance / 2.0;
                    let move_y = overlap * dy / distance / 2.0;
                    particles[i].x += move_x;
                    particles[i].y += move_y;
                    particles[j].x -= move_x;
                    particles[j].y -= move_y;
                }
            }
        }

        for particle in &mut particles {
            particle.update(friction, current_width, current_height, circle_radius);
        }

        window.draw_2d(&event, |c, g, _| {
            clear([0.0; 4], g);
            if show_circle {
                for particle in &particles {
                    ellipse(
                        particle.color,
                        [
                            particle.x - circle_radius,
                            particle.y - circle_radius,
                            2.0 * circle_radius,
                            2.0 * circle_radius,
                        ],
                        c.transform,
                        g,
                    );
                }
            }

            if trace_line {
                if bezier {
                    for i in 0..particles.len() - 1 {
                        let p0 = [particles[i].x, particles[i].y];
                        let p1 = [
                            (particles[i].x + particles[i + 1].x) / 2.0,
                            (particles[i].y + particles[i + 1].y) / 2.0,
                        ];
                        let p2 = [particles[i + 1].x, particles[i + 1].y];
                        draw_bezier_curve(p0, p1, p2, [1.0, 0.0, 0.0, 1.0], 2.0, c.transform, g);
                    }
                } else {
                    for i in 0..particles.len() - 1 {
                        line(
                            [1.0, 0.0, 0.0, 1.0], // color: red
                            2.0,                  // thickness
                            [
                                particles[i].x,
                                particles[i].y,
                                particles[i + 1].x,
                                particles[i + 1].y,
                            ], // line coordinates
                            c.transform,
                            g,
                        );
                    }
                }
            }
        });
    }
}

fn draw_bezier_curve<G: Graphics>(
    p0: [f64; 2],
    p1: [f64; 2],
    p2: [f64; 2],
    color: [f32; 4],
    thickness: f64,
    transform: [[f64; 3]; 2],
    g: &mut G,
) {
    let steps = 100;
    let amplitude = 15.0; // Amplitude of the sine wave
    let frequency = 3.0; // Frequency of the sine wave

    for i in 0..steps {
        let t1 = i as f64 / steps as f64;
        let t2 = (i + 1) as f64 / steps as f64;

        let x1 = (1.0 - t1).powi(2) * p0[0] + 2.0 * (1.0 - t1) * t1 * p1[0] + t1.powi(2) * p2[0];
        let y1 = (1.0 - t1).powi(2) * p0[1] + 2.0 * (1.0 - t1) * t1 * p1[1] + t1.powi(2) * p2[1];

        let x2 = (1.0 - t2).powi(2) * p0[0] + 2.0 * (1.0 - t2) * t2 * p1[0] + t2.powi(2) * p2[0];
        let y2 = (1.0 - t2).powi(2) * p0[1] + 2.0 * (1.0 - t2) * t2 * p1[1] + t2.powi(2) * p2[1];

        let wave1 = amplitude * (frequency * t1 * 2.0 * PI).sin();
        let wave2 = amplitude * (frequency * t2 * 2.0 * PI).sin();

        line(
            color,
            thickness,
            [x1 + wave1, y1 + wave1, x2 + wave2, y2 + wave2],
            transform,
            g,
        );
    }
}
