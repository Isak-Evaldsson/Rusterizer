mod vector;

// sdl
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() {
    // Triangle
    let p0 = vector::vec3(0.0, 0.0, 0.0);
    let p0 = vector::vec3(0.0, 0.0, 1.0);
    let p0 = vector::vec3(0.0, 1.0, 1.0);

    // sdl setup
    let sdl_context = sdl2::init().unwrap();
    let video_subsysem = sdl_context.video().unwrap();

    let window = video_subsysem.window("Rusteriser", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // drawing loop
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                // quit on window exit click
                Event::Window { timestamp: _, window_id: _, win_event: WindowEvent::Close } => {
                    break 'main;
                },
                _ => {}
            }
        }
    }
}
