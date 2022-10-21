mod frame_buffer;
mod linalg;

// sdl
extern crate sdl2;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Canvas, Texture};
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::time::{Duration, SystemTime};

use linalg::Vec3;

// Screen cosntants
const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

fn main() {
    // Triangle
    let p0 = Vec3::new(0.0, 1.0, 1.0);
    let p1 = Vec3::new(1.0, 1.0, 1.0);
    let p2 = Vec3::new(1.0, 0.0, 1.0);

    // Frame buffer
    let mut buff = frame_buffer::FrameBuffer::new(WIDTH as usize, HEIGHT as usize);

    // sdl setup
    let sdl_context = sdl2::init().unwrap();
    let video_subsysem = sdl_context.video().unwrap();

    let window = video_subsysem
        .window("Rusteriser", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("failed to build windows canvas");
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH, HEIGHT)
        .expect("Could not create texture");

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut time = SystemTime::now();

    // drawing loop
    'main: loop {
        println!("ms: {}", time.elapsed().ok().unwrap().as_millis());
        time = SystemTime::now();

        for event in event_pump.poll_iter() {
            match event {
                // quit on window exit click
                Event::Window {
                    timestamp: _,
                    window_id: _,
                    win_event: WindowEvent::Close,
                } => {
                    break 'main;
                }
                _ => {}
            }
        }

        // Rasterise triangle
        let mut x = 0.0;
        let mut y = 0.0;
        let dx = 1.0/(WIDTH as f32);
        let dy = 1.0/(HEIGHT as f32);

        // edge equations
        let n_1 = p0.cross(&p1);
        let n_2 = p2.cross(&p0);
        let n_3 = p1.cross(&p2);

        for i in 0..(WIDTH as usize) {
            for j in 0..(HEIGHT as usize) {
                let p = Vec3::new(x, y, 1.0);

                // edge test
                if p.dot(&n_1) < 0.0 && p.dot(&n_3) < 0.0 && p.dot(&n_2) < 0.0 {
                    buff.set(i * (WIDTH as usize) + j, frame_buffer::RGB::rgb(255, 0, 0));
                }

                x += dx;
            }
            x = 0.0;
            y += dy;
        }

        // Copy frame buffer into texture
        buff.copy_to_texture(&mut texture);

        // Render texture
        canvas.copy(&texture, None, None);
        canvas.present();
    }
}
