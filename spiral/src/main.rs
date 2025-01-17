use piston_window::*;
use rand::Rng;
use std::env;

fn print_help() {
    println!("Usage: cargo run -- [OPTIONS]");
    println!("Options:");
    println!("  radius1=<radius>                Radius of circles (default: 10.0)");
    println!("  angle_increment=<increment>     Angle increment for spiral (default: 0.05)");
    println!("  spiral_radius_increment=<increment> Spiral radius increment (default: 0.5)");
    println!("  connect_line=<true/false>       Connect circles with lines (default: false)");
    println!("  ellipse_duration=<DURATION>     Sets the duration of each ellipse in seconds (default: 1)");
    println!("  help                            Prints this help message");
}

#[derive(Clone)]
struct Spiral {
    x: f64,
    y: f64,
    angle: f64,
    radius: f64,
    color: [f32; 4],
    direction: f64,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "help") {
        print_help();
        return;
    }

    const DEFAULT_RADIUS1: f64 = 10.0;
    const DEFAULT_ANGLE_INCREMENT: f64 = 0.05;
    const DEFAULT_SPIRAL_RADIUS_INCREMENT: f64 = 0.5;
    const DEFAULT_CONNECT_LINE: bool = false;
    const DEFAULT_ELLIPSE_DURATION: u64 = 1;

    let radius1 = env::args()
        .find(|arg| arg.starts_with("radius1="))
        .and_then(|arg| arg.split('=').nth(1).map(|val| val.to_string()))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_RADIUS1);

    let angle_increment = env::args()
        .find(|arg| arg.starts_with("angle_increment="))
        .and_then(|arg| arg.split('=').nth(1).map(|val| val.to_string()))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_ANGLE_INCREMENT);

    let spiral_radius_increment = env::args()
        .find(|arg| arg.starts_with("spiral_radius_increment="))
        .and_then(|arg| arg.split('=').nth(1).map(|val| val.to_string()))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_SPIRAL_RADIUS_INCREMENT);

    let connect_line = env::args()
        .find(|arg| arg.starts_with("connect_line="))
        .and_then(|arg| arg.split('=').nth(1).map(|val| val.to_string()))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_CONNECT_LINE);

    let ellipse_duration = env::args()
        .find(|arg| arg.starts_with("ellipse_duration="))
        .and_then(|arg| arg.split('=').nth(1).map(|val| val.to_string()))
        .and_then(|val| val.parse().ok())
        .unwrap_or(DEFAULT_ELLIPSE_DURATION);

    let mut window: PistonWindow = WindowSettings::new("Spiral Particle", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let colors: [[f32; 4]; 8] = [
        [1.0, 0.0, 0.0, 1.0], // Red
        [0.0, 1.0, 0.0, 1.0], // Green
        [0.0, 0.0, 1.0, 1.0], // Blue
        [1.0, 1.0, 0.0, 1.0], // Yellow
        [1.0, 0.0, 1.0, 1.0], // Magenta
        [0.0, 1.0, 1.0, 1.0], // Cyan
        [0.5, 0.5, 0.5, 1.0], // Gray
        [1.0, 0.5, 0.0, 1.0], // Orange
    ];

    let mut spirals: Vec<Spiral> = Vec::new();
    let mut mouse_pos: [f64; 2] = [0.0, 0.0];

    while let Some(event) = window.next() {
        if let Some(pos) = event.mouse_cursor_args() {
            mouse_pos = pos;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            println!("Mouse clicked at: {:?}", mouse_pos); // Debug print
            let color = colors[rand::thread_rng().gen_range(0..colors.len())];
            let direction = if rand::thread_rng().gen_bool(0.5) {
                1.0
            } else {
                -1.0
            };
            spirals.push(Spiral {
                x: mouse_pos[0],
                y: mouse_pos[1],
                angle: 0.0,
                radius: 0.0,
                color,
                direction,
            });
            println!("Spiral created at: {:?}", mouse_pos); // Debug print
        }

        window.draw_2d(&event, |c, g, _| {
            clear([1.0; 4], g);

            // Draw lines first
            for i in 0..spirals.len() {
                if connect_line && i > 0 {
                    let (left, right) = spirals.split_at_mut(i);
                    let spiral = &right[0];
                    let prev_spiral = &left[i - 1];
                    let x = spiral.x + spiral.radius * spiral.angle.cos();
                    let y = spiral.y + spiral.radius * spiral.angle.sin();
                    let prev_x = prev_spiral.x + prev_spiral.radius * prev_spiral.angle.cos();
                    let prev_y = prev_spiral.y + prev_spiral.radius * prev_spiral.angle.sin();
                    line_from_to(
                        [0.0, 0.0, 0.0, 1.0],
                        1.0,
                        [prev_x, prev_y],
                        [x, y],
                        c.transform,
                        g,
                    );
                }
            }

            // Draw circles
            for i in 0..spirals.len() {
                let (_left, right) = spirals.split_at_mut(i);
                let spiral = &mut right[0];
                let x = spiral.x + spiral.radius * spiral.angle.cos();
                let y = spiral.y + spiral.radius * spiral.angle.sin();
                ellipse(
                    spiral.color,
                    [x - radius1 / 2.0, y - radius1 / 2.0, radius1, radius1],
                    c.transform,
                    g,
                );

                spiral.angle += angle_increment * spiral.direction;
                spiral.radius += spiral_radius_increment;
            }

            spirals.retain(|spiral| {
                let x = spiral.x + spiral.radius * spiral.angle.cos();
                let y = spiral.y + spiral.radius * spiral.angle.sin();
                x >= 0.0 && x <= 800.0 && y >= 0.0 && y <= 600.0
            });
        });

        std::thread::sleep(std::time::Duration::from_millis(ellipse_duration));
    }
}
