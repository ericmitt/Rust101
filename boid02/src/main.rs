extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;
use std::env;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 600.0;
const BOID_DISTANCE: f64 = 8.0;
const BOID_SAULT: f64 = 1.9;
const BOID_SIZE: f64 = 12.0;
const HEIGHT_OBS: f64 = 140.0;
const WIDTH_OBS: f64 = 140.0;

#[derive(Clone)]
struct Boid {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

#[derive(Clone)]
struct Obstacle {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Boid {
    pub fn update(
        &mut self,
        target_x: f64,
        target_y: f64,
        boids: &[Boid],
        obstacles: &[Obstacle],
        max_angle: f64,
        boid_speed: f64,
    ) {
        let ix = self.x;
        let iy = self.y;
        let dx = target_x - self.x;
        let dy = target_y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let direction_x = dx / distance;
        let direction_y = dy / distance;

        let target_angle = direction_y.atan2(direction_x).to_degrees();
        let current_angle = self.vy.atan2(self.vx).to_degrees();
        let angle_diff = target_angle - current_angle;

        let clamped_angle_diff = angle_diff.clamp(-max_angle, max_angle);
        let new_angle = current_angle + clamped_angle_diff;

        self.vx = new_angle.to_radians().cos() * boid_speed;
        self.vy = new_angle.to_radians().sin() * boid_speed;

        self.x += self.vx;
        self.y += self.vy;

        if self.x < 0.0 || self.x > WIDTH {
            self.vx = -self.vx;
        }
        if self.y < 0.0 || self.y > HEIGHT {
            self.vy = -self.vy;
        }

        for boid in boids {
            if (self.x - boid.x).abs() < BOID_DISTANCE || (self.y - boid.y).abs() < BOID_DISTANCE {
                let mut rng = rand::thread_rng();
                self.x += rng.gen_range(BOID_SAULT * -1.0..BOID_SAULT);
                self.y += rng.gen_range(BOID_SAULT * -1.0..BOID_SAULT);
            }
        }

        for obs in obstacles {
            if self.x > obs.x && self.x < obs.x + obs.w {
                if self.y > obs.y && self.y < obs.y + obs.h {
                    self.x = ix;
                    if self.y > obs.y + HEIGHT_OBS / 2.0 {
                        self.y = iy + boid_speed;
                    } else {
                        self.y = iy - boid_speed;
                    }
                }
            }
        }
    }

    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        self.x = rng.gen_range(0.0..WIDTH);
        self.y = rng.gen_range(0.0..HEIGHT);
    }

    fn draw(&self, c: Context, g: &mut G2d) {
        ellipse(
            [1.0, 0.0, 0.0, 1.0],
            [self.x, self.y, BOID_SIZE, BOID_SIZE],
            c.transform,
            g,
        );
    }
}

impl Obstacle {
    fn draw(&self, c: Context, g: &mut G2d) {
        rectangle(
            [0.0, 0.0, 1.0, 1.0],
            [self.x, self.y, self.w, self.h],
            c.transform,
            g,
        );
    }

    fn _randomize(&mut self) {
        let mut rng = rand::thread_rng();
        self.x = rng.gen_range(15.0..WIDTH);
        self.y = rng.gen_range(15.0..HEIGHT);
        self.w = rng.gen_range(15.0..WIDTH_OBS);
        self.h = rng.gen_range(15.0..HEIGHT_OBS);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("help")) {
        println!("Usage: cargo run -- [OPTIONS]");
        println!("Options:");
        println!("  num_boids=<number>          Number of boids (default: 30)");
        println!("  boid_speed=<speed>          Speed of boids (default: 4.0)");
        println!("  num_obs=<number>            Number of obstacles (default: 30)");
        println!("  max_angle=<angle>           Maximum angle for boid rotation (default: 30.0)");
        return;
    }

    let num_boids = args
        .iter()
        .find(|arg| arg.starts_with("num_boids="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(30);

    let boid_speed = args
        .iter()
        .find(|arg| arg.starts_with("boid_speed="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(4.0);

    let num_obs = args
        .iter()
        .find(|arg| arg.starts_with("num_obs="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(30);

    let max_angle = args
        .iter()
        .find(|arg| arg.starts_with("max_angle="))
        .and_then(|arg| arg.split('=').nth(1))
        .and_then(|val| val.parse().ok())
        .unwrap_or(30.0);

    let mut window: PistonWindow = WindowSettings::new("Boids", [WIDTH as u32, HEIGHT as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut rng = rand::thread_rng();
    let mut boids: Vec<Boid> = (0..num_boids)
        .map(|_| Boid {
            x: rng.gen_range(0.0..WIDTH),
            y: rng.gen_range(0.0..HEIGHT),
            vx: boid_speed,
            vy: boid_speed,
        })
        .collect();

    let obstacles: Vec<Obstacle> = (0..num_obs)
        .map(|_| Obstacle {
            x: rng.gen_range(5.0..WIDTH),
            y: rng.gen_range(5.0..HEIGHT),
            w: rng.gen_range(5.0..WIDTH_OBS),
            h: rng.gen_range(5.0..HEIGHT_OBS),
        })
        .collect();

    println!("Boids and Obstacles created");

    while let Some(event) = window.next() {
        if let Some(mouse_pos) = event.mouse_cursor_args() {
            let obstacles_copy = obstacles.clone();
            for i in 0..boids.len() {
                let (left, right) = boids.split_at_mut(i);
                let boid = &mut right[0];
                boid.update(
                    mouse_pos[0],
                    mouse_pos[1],
                    left,
                    &obstacles_copy,
                    max_angle,
                    boid_speed,
                );
            }
        }

        if let Some(_button) = event.press_args() {
            for boid in &mut boids {
                boid.randomize();
            }
        }

        window.draw_2d(&event, |c, g, _| {
            clear([1.0; 4], g);
            for obs in &obstacles {
                obs.draw(c, g);
            }
            for boid in &boids {
                boid.draw(c, g);
            }
        });
    }
}
