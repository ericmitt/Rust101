use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use piston_window::*;
use rand::Rng;
use rustfft::FftPlanner;
use std::env;
use std::sync::{Arc, Mutex};

fn capture_audio(decibels: Arc<Mutex<f32>>, pitch: Arc<Mutex<f32>>, duration: u64) {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input device available");
    let config = device
        .default_input_config()
        .expect("Failed to get default input format");

    let stream_config = config.clone().into();

    let stream = device
        .build_input_stream(
            &stream_config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let rms = (data.iter().map(|&x| x * x).sum::<f32>() / data.len() as f32).sqrt();
                let mut decibels = decibels.lock().unwrap();
                *decibels = 20.0 * rms.log10();

                // Analyse du pitch
                let mut planner = FftPlanner::new();
                let fft = planner.plan_fft_forward(data.len());
                let mut buffer: Vec<_> = data
                    .iter()
                    .map(|&x| rustfft::num_complex::Complex::new(x, 0.0))
                    .collect();
                fft.process(&mut buffer);

                let mut max_magnitude = 0.0;
                let mut max_index = 0;
                for (i, &complex) in buffer.iter().enumerate() {
                    let magnitude = complex.norm();
                    if magnitude > max_magnitude {
                        max_magnitude = magnitude;
                        max_index = i;
                    }
                }

                let sample_rate = stream_config.sample_rate.0 as f32;
                let frequency = max_index as f32 * sample_rate / data.len() as f32;

                let mut pitch = pitch.lock().unwrap();
                *pitch = frequency;
            },
            move |err| {
                eprintln!("Error: {:?}", err);
            },
            None,
        )
        .unwrap();

    stream.play().unwrap();

    // Boucle infinie pour capturer l'audio en continu
    loop {
        std::thread::sleep(std::time::Duration::from_secs(duration));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    const DEFAULT_DURATION: u64 = 30;

    let duration = args
        .get(1)
        .unwrap_or(&DEFAULT_DURATION.to_string())
        .parse::<u64>()
        .expect("Invalid duration");

    let decibels = Arc::new(Mutex::new(0.0));
    let pitch = Arc::new(Mutex::new(0.0));
    let decibels_clone = Arc::clone(&decibels);
    let pitch_clone = Arc::clone(&pitch);

    std::thread::spawn(move || {
        capture_audio(decibels_clone, pitch_clone, duration);
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
        let pitch = *pitch.lock().unwrap();
        let radius = ((decibels + 60.0).max(0.0) * 20.0) as f64; // Ajuster l'échelle pour rendre la taille plus sensible aux décibels

        if frame_count % 10 == 0 {
            color_index = rand::thread_rng().gen_range(0..colors.len());
        }

        let color = colors[color_index];

        // ...existing code...
        let Size { width, height } = window.draw_size();
        window.draw_2d(&event, |c, g, _| {
            clear([0.0; 4], g);

            let center_x = (width as f64 / 2.0).floor();
            let center_y = (height as f64 / 2.0).floor();

            // Change the shape based on the pitch
            if pitch < 200.0 {
                ellipse(
                    color,
                    [
                        center_x - radius, // Left edge
                        center_y - radius, // Top edge
                        radius * 2.0,      // Width
                        radius * 2.0,      // Height
                    ],
                    c.transform,
                    g,
                );
            } else if pitch < 400.0 {
                rectangle(
                    color,
                    [
                        center_x - radius, // Left edge
                        center_y - radius, // Top edge
                        radius * 2.0,      // Width
                        radius * 2.0,      // Height
                    ],
                    c.transform,
                    g,
                );
            } else {
                let points = [
                    [center_x, center_y - radius],          // Top vertex
                    [center_x - radius, center_y + radius], // Bottom left vertex
                    [center_x + radius, center_y + radius], // Bottom right vertex
                ];
                let points_f64: Vec<[f64; 2]> = points.iter().map(|&p| [p[0], p[1]]).collect();
                polygon(color, &points_f64, c.transform, g);
            }
        });

        frame_count += 1;
        std::thread::sleep(std::time::Duration::from_millis(duration));
    }
}
