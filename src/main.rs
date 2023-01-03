mod frame_buffer;
mod linalg;
mod mesh;
mod rasterizer;

// sdl
extern crate sdl2;

use frame_buffer::RGB;
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

// Configuration
const FPS_COUNTER: bool = false;

fn main() {
    // Triangle
    let p0 = Vec3::new(0.0, 1.0, 1.0);
    let p1 = Vec3::new(1.0, 1.0, 1.0);
    let p2 = Vec3::new(1.0, 0.0, 1.0);

    let p3 = Vec3::new(0.3, 0.7, 1.0);
    let p4 = Vec3::new(0.7, 0.7, 1.0);
    let p5 = Vec3::new(0.7, 0.3, 1.0);

    let t1 = mesh::Triangle::new(p0, p1, p2, RGB::rgb(255, 0, 0));
    let t2 = mesh::Triangle::new(p3, p4, p5, RGB::rgb(0, 255, 0));

    let mesh = mesh::Mesh::new(vec![t1, t2]);

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
        if FPS_COUNTER {
            println!("ms: {}", time.elapsed().ok().unwrap().as_millis());
            time = SystemTime::now();
        }

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

        rasterizer::rasterize(&mesh, &mut buff, WIDTH, HEIGHT);

        // Copy frame buffer into texture
        buff.copy_to_texture(&mut texture);

        // Render texture
        canvas.copy(&texture, None, None);
        canvas.present();
    }
}
