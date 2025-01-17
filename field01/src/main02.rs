extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};
use std::env;

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

    fn update(&mut self, friction: f64) {
        self.x += self.dx * self.speed;
        self.y += self.dy * self.speed;

        // Apply friction
        self.speed *= 1.0 - friction;

        // Bounce off walls
        if self.x < DEFAULT_CIRCLE_RADIUS || self.x > DEFAULT_WIDTH as f64 - DEFAULT_CIRCLE_RADIUS {
            self.dx = -self.dx;
        }
        if self.y < DEFAULT_CIRCLE_RADIUS || self.y > DEFAULT_HEIGHT as f64 - DEFAULT_CIRCLE_RADIUS {
            self.dy = -self.dy;
        }
    }

    fn set_random_direction_and_speed(&mut self, max_speed: f64) {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..2.0 * PI);
        self.dx = angle.cos();
        self.dy = angle.sin();
        self.speed = rng.gen_range(0.0..max_speed);
    }

    fn apply_impulse(&mut self, impulse: f64) {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..2.0 * PI);
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
        println!("  trace_line=<true/false>     Enable mouse move impulse (default: false)");
        return;
    }

    let width = args.iter().find(|arg| arg.starts_with("width="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_WIDTH);

    let height = args.iter().find(|arg| arg.starts_with("height="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_HEIGHT);

    let circle_radius = args.iter().find(|arg| arg.starts_with("circle_radius="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_CIRCLE_RADIUS);

    let rows = args.iter().find(|arg| arg.starts_with("rows="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_ROWS);

    let cols = args.iter().find(|arg| arg.starts_with("cols="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_COLS);

    let max_speed = args.iter().find(|arg| arg.starts_with("max_speed="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_MAX_SPEED);

    let friction = args.iter().find(|arg| arg.starts_with("friction="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_FRICTION);

    let dev_angle = args.iter().find(|arg| arg.starts_with("dev_angle="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_DEV_ANGLE);

    let mouse_move = args.iter().find(|arg| arg.starts_with("mouse_move="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_MOUSE_MOVE);

    let trace_line = args.iter().find(|arg| arg.starts_with("trace_line="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_TRACE_LINE);

    let mut window: PistonWindow = WindowSettings::new("Particle Field", [width, height])
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

        for i in 0..particles.len() {
            for j in (i + 1)..particles.len() {
                let dx = particles[i].x - particles[j].x;
                let dy = particles[i].y - particles[j].y;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance < 2.0 * circle_radius {
                    // Collision detected, transfer energy and apply deviation angle
                    let total_energy = particles[i].speed + particles[j].speed;
                    particles[i].speed = total_energy / 2.0;
                    particles[j].speed = total_energy / 2.0;

                    // Apply deviation angle
                    let angle = dev_angle;
                    let new_dx_i = particles[i].dx * angle.cos() - particles[i].dy * angle.sin();
                    let new_dy_i = particles[i].dx * angle.sin() + particles[i].dy * angle.cos();
                    particles[i].dx = new_dx_i;
                    particles[i].dy = new_dy_i;

                    let new_dx_j = particles[j].dx * angle.cos() - particles[j].dy * angle.sin();
                    let new_dy_j = particles[j].dx * angle.sin() + particles[j].dy * angle.cos();
                    particles[j].dx = new_dx_j;
                    particles[j].dy = new_dy_j;

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
            particle.update(friction);
        }

        window.draw_2d(&event, |c, g, _| {
            clear([1.0; 4], g);
            for particle in &particles {
                ellipse(particle.color, [particle.x - circle_radius, particle.y - circle_radius, 2.0 * circle_radius, 2.0 * circle_radius], c.transform, g);
            }
            
            if trace_line {
            for i in 0..particles.len() - 1 {
                line([1.0, 0.0, 0.0, 1.0], // color: red
                     2.0, // thickness
                     [particles[i].x, particles[i].y, particles[i + 1].x, particles[i + 1].y], // line coordinates
                     c.transform,
                     g);
            }
        }
        });
    }
}
   
    
