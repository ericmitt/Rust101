extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const CIRCLE_RADIUS: f64 = 10.0;
const ROWS: usize = 15;
const COLS: usize = 20;
const MAX_SPEED: f64 = 5.0;
const GRAVITY: f64 = 9.8;

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

    fn update(&mut self) {
        self.x += self.dx * self.speed;
        self.y += self.dy * self.speed;

        // Apply gravity
        self.dy += GRAVITY * 0.01;

        // Bounce off walls
        if self.x < CIRCLE_RADIUS || self.x > WIDTH as f64 - CIRCLE_RADIUS {
            self.dx = -self.dx;
        }
        if self.y < CIRCLE_RADIUS || self.y > HEIGHT as f64 - CIRCLE_RADIUS {
            self.dy = -self.dy;
        }
    }

    fn set_random_direction_and_speed(&mut self) {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..2.0 * PI);
        self.dx = angle.cos();
        self.dy = angle.sin();
        self.speed = rng.gen_range(0.0..MAX_SPEED);
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Particle Field", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut particles = vec![];
    for i in 0..ROWS {
        for j in 0..COLS {
            let x = j as f64 * (2.0 * CIRCLE_RADIUS) + CIRCLE_RADIUS;
            let y = i as f64 * (2.0 * CIRCLE_RADIUS) + CIRCLE_RADIUS;
            let color = [rand::random(), rand::random(), rand::random(), 1.0];
            particles.push(Particle::new(x, y, color));
        }
    }

    let target = Arc::new(Mutex::new((0.0, 0.0)));

    while let Some(event) = window.next() {
        if let Some(mouse_pos) = event.mouse_cursor_args() {
            let mut target = target.lock().unwrap();
            *target = (mouse_pos[0], mouse_pos[1]);
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            let target = target.lock().unwrap();
            for particle in &mut particles {
                let dx = particle.x - target.0;
                let dy = particle.y - target.1;
                if dx * dx + dy * dy < CIRCLE_RADIUS * CIRCLE_RADIUS {
                    particle.set_random_direction_and_speed();
                }
            }
        }

        for i in 0..particles.len() {
            for j in (i + 1)..particles.len() {
                let dx = particles[i].x - particles[j].x;
                let dy = particles[i].y - particles[j].y;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance < 2.0 * CIRCLE_RADIUS {
                    // Collision detected, transfer energy
                    let _total_mass = 1.0; // Assuming equal mass for simplicity
                    let total_energy = particles[i].speed + particles[j].speed;
                    particles[i].speed = total_energy / 2.0;
                    particles[j].speed = total_energy / 2.0;
                }
            }
        }

        for particle in &mut particles {
            particle.update();
        }

        window.draw_2d(&event, |c, g, _| {
            clear([1.0; 4], g);
            for particle in &particles {
                ellipse(particle.color, [particle.x - CIRCLE_RADIUS, particle.y - CIRCLE_RADIUS, 2.0 * CIRCLE_RADIUS, 2.0 * CIRCLE_RADIUS], c.transform, g);
            }
        });
    }
}
