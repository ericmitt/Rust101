use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use piston_window::*;
use rand::Rng;
use std::sync::{Arc, Mutex};

fn capture_audio(decibels: Arc<Mutex<f32>>) {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("No input device available");
    let config = device.default_input_config().expect("Failed to get default input format");

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let rms = (data.iter().map(|&x| x * x).sum::<f32>() / data.len() as f32).sqrt();
            let mut decibels = decibels.lock().unwrap();
            *decibels = 20.0 * rms.log10();
        },
        move |err| {
            eprintln!("Error: {:?}", err);
        },
        None,
    ).unwrap();

    stream.play().unwrap();

    // Boucle infinie pour capturer l'audio en continu
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn main() {
    let decibels = Arc::new(Mutex::new(0.0));
    let decibels_clone = Arc::clone(&decibels);

    std::thread::spawn(move || {
        capture_audio(decibels_clone);
    });

    let mut window: PistonWindow = WindowSettings::new("Audio Visualizer", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let colors = [
        [1.0, 0.0, 0.0, 1.0], // Red
        [0.0, 1.0, 0.0, 1.0], // Green
        [0.0, 0.0, 1.0, 1.0], // Blue
        [1.0, 1.0, 0.0, 1.0], // Yellow
        [1.0, 0.0, 1.0, 1.0], // Magenta
        [0.0, 1.0, 1.0, 1.0], // Cyan
        [0.5, 0.5, 0.5, 1.0], // Gray
        [1.0, 0.5, 0.0, 1.0], // Orange
    ];

    let mut color_index = 0;
    let mut frame_count = 0;

    while let Some(event) = window.next() {
        let decibels = *decibels.lock().unwrap();
        let radius = (decibels + 60.0).max(0.0) * 10.0; // Ajuster l'échelle pour rendre la taille plus sensible aux décibels

        if frame_count % 10 == 0 {
            color_index = rand::thread_rng().gen_range(0..colors.len());
        }

        let color = colors[color_index];

        window.draw_2d(&event, |c, g, _| {
            clear([1.0; 4], g);
            ellipse(color, [400.0 - radius / 2.0, 300.0 - radius / 2.0, radius, radius].map(|v| v as f64), c.transform, g);
        });

        frame_count += 1;
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
