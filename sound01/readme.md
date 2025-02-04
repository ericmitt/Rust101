# Audio Visualizer

A real-time audio visualization program written in Rust that displays geometric shapes based on audio input.

## Features

- Real-time audio input processing
- Dynamic shape visualization based on:
  - Audio volume (decibels) controlling shape size
  - Pitch controlling shape type:
    - Low pitch (< 200 Hz): Circle
    - Medium pitch (200-400 Hz): Square
    - High pitch (> 400 Hz): Triangle
- Color changes every 10 frames

## Prerequisites

- Rust (latest stable version)
- Working microphone
- Required dependencies:
  - cpal (audio capture)
  - piston_window (graphics)
  - rustfft (audio analysis)
  - rand (color randomization)

## Installation

1. Clone the repository
2. Navigate to the project directory
3. Run:
```bash
cargo run 

make noise to see animation