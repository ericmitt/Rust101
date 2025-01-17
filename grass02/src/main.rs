extern crate piston_window;
use piston_window::*;
use rand::Rng;
use std::env;

const DEFAULT_WORLD_SIZE: usize = 100;
const DEFAULT_CELL_SIZE: f64 = 10.0;
const DEFAULT_MAX_SEEDS: usize = 1;
const DEFAULT_REPRODUCE_AGE: u32 = 10;
const DEFAULT_DEATH_AGE: u32 = 50;
const DEFAULT_REPRO_RADIUS: isize = 3;

fn get_param<T: std::str::FromStr>(name: &str, default: T) -> T {
    env::args()
        .find(|arg| arg.starts_with(name))
        .and_then(|arg| arg.split('=').nth(1).map(|val| val.to_string()))
        .and_then(|val| val.parse().ok())
        .unwrap_or(default)
}

fn print_help() {
    println!("Usage: grass_simulation [OPTIONS]");
    println!("Options:");
    println!("  WORLD_SIZE=<usize>       Set the size of the world (default: 100)");
    println!("  CELL_SIZE=<f64>          Set the size of each cell (default: 10.0)");
    println!("  MAX_SEEDS=<usize>        Set the maximum number of seeds (default: 1)");
    println!("  REPRODUCE_AGE=<u32>      Set the age at which grass can reproduce (default: 10)");
    println!("  DEATH_AGE=<u32>          Set the age at which grass dies (default: 50)");
    println!("  REPRO_RADIUS=<isize>     Set the reproduction radius (default: 3)");
    println!("  help                     Print this help message");
}

#[derive(Clone, Copy)]
struct Grass {
    age: u32,
    death_age: u32,
}

impl Grass {
    fn new(death_age: u32) -> Self {
        let mut rng = rand::thread_rng();
        let variation = rng.gen_range(0.8..=1.2);
        Grass {
            age: 0,
            death_age: (death_age as f64 * variation) as u32,
        }
    }

    fn update(&mut self) {
        self.age += 1;
    }

    fn can_reproduce(&self, reproduce_age: u32) -> bool {
        self.age >= reproduce_age && self.age < self.death_age
    }

    fn is_dead(&self) -> bool {
        self.age >= self.death_age
    }
}

struct World {
    grid: Vec<Vec<Option<Grass>>>,
}

impl World {
    fn new(size: usize) -> Self {
        World {
            grid: vec![vec![None; size]; size],
        }
    }

    fn update(
        &mut self,
        max_seeds: usize,
        reproduce_age: u32,
        death_age: u32,
        repro_radius: isize,
    ) {
        let mut rng = rand::thread_rng();
        for x in 0..self.grid.len() {
            for y in 0..self.grid.len() {
                if let Some(grass) = &mut self.grid[x][y] {
                    grass.update();
                    if grass.is_dead() {
                        self.grid[x][y] = None;
                    } else if grass.can_reproduce(reproduce_age) {
                        for _ in 0..max_seeds {
                            let dx = rng.gen_range(-1 * repro_radius..=repro_radius);
                            let dy = rng.gen_range(-1 * repro_radius..=repro_radius);
                            let nx = (x as isize + dx) as usize;
                            let ny = (y as isize + dy) as usize;
                            if nx < self.grid.len()
                                && ny < self.grid.len()
                                && self.grid[nx][ny].is_none()
                            {
                                self.grid[nx][ny] = Some(Grass::new(death_age));
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    fn draw(&self, c: Context, g: &mut G2d, cell_size: f64, reproduce_age: u32) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid.len() {
                if let Some(grass) = &self.grid[x][y] {
                    let color = if grass.age < reproduce_age {
                        [0.0, 1.0, 0.0, 1.0] // Clear green
                    } else {
                        [0.0, 0.5, 0.0, 1.0] // Dark green
                    };
                    rectangle(
                        color,
                        [
                            x as f64 * cell_size,
                            y as f64 * cell_size,
                            cell_size,
                            cell_size,
                        ],
                        c.transform,
                        g,
                    );
                }
            }
        }
    }
}

fn main() {
    if env::args().any(|arg| arg == "help") {
        print_help();
        return;
    }

    let world_size = get_param("WORLD_SIZE", DEFAULT_WORLD_SIZE);
    let cell_size = get_param("CELL_SIZE", DEFAULT_CELL_SIZE);
    let max_seeds = get_param("MAX_SEEDS", DEFAULT_MAX_SEEDS);
    let reproduce_age = get_param("REPRODUCE_AGE", DEFAULT_REPRODUCE_AGE);
    let death_age = get_param("DEATH_AGE", DEFAULT_DEATH_AGE);
    let repro_radius = get_param("REPRO_RADIUS", DEFAULT_REPRO_RADIUS);

    let mut window: PistonWindow = WindowSettings::new("Grass Simulation", [1000, 700])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut world = World::new(world_size);

    let mut mouse_pos = [50.0, 60.0];
    while let Some(event) = window.next() {
        if let Some(pos) = event.mouse_cursor_args() {
            mouse_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            let x = (mouse_pos[0] as f64 / cell_size) as usize;
            let y = (mouse_pos[1] as f64 / cell_size) as usize;
            if x < world_size && y < world_size {
                world.grid[x][y] = Some(Grass::new(death_age));
            }
        }

        if let Some(_) = event.update_args() {
            world.update(max_seeds, reproduce_age, death_age, repro_radius);
        }

        window.draw_2d(&event, |c, g, _| {
            clear([1.0; 4], g);
            world.draw(c, g, cell_size, reproduce_age);
            // text::Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
            //     .draw(
            //         &format!("Step: {}, Grass count: {}", step, world.grid.iter().flatten().filter(|&&cell| cell.is_some()).count()),
            //         &mut window.factory,
            //         c.transform.trans(10.0, 980.0),
            //         g,
            //     )
            //     .unwrap();
        });
    }
}
