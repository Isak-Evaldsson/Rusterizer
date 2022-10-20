mod frame_buffer;
mod linalg;

// sdl
extern crate sdl2;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture};
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::time::{SystemTime, Duration};

use linalg::Vec3;

// Screen cosntants
const WIDTH: u32= 800;
const HEIGHT: u32 = 600;

fn main() {

    // Triangle
    let p0 = Vec3::new(0.0, 0.0, 0.0);
    let p0 = Vec3::new(0.0, 0.0, 1.0);
    let p0 = Vec3::new(0.0, 1.0, 1.0);

    // Frame buffer
    let mut buff = frame_buffer::FrameBuffer::new(WIDTH as usize, HEIGHT as usize);

    // sdl setup
    let sdl_context = sdl2::init().unwrap();
    let video_subsysem = sdl_context.video().unwrap();

    let window = video_subsysem.window("Rusteriser", WIDTH, HEIGHT)
    .position_centered()
    .build()
    .unwrap();

    let mut canvas = window.into_canvas()
    .build()
    .expect("failed to build windows canvas");
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, WIDTH, HEIGHT)
    .expect("Could not create texture");

    let mut event_pump = sdl_context.event_pump().unwrap();

    let time = SystemTime::now();

    // drawing loop
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                // quit on window exit click
                Event::Window { timestamp: _, window_id: _, win_event: WindowEvent::Close } =>  {
                    break 'main;
                },
                _ => {}
            }
        }

        // Paint some strips verifying working frame buffer
        let t = time.elapsed().ok().unwrap().as_secs();

        for i in 0..buff.size() {
            if (t % 2 == 0) {
                buff.set(i, frame_buffer::RGB::rgb(0, 0, 255));
            } else {
                if (i % 100 > 50) {
                    buff.set(i, frame_buffer::RGB::rgb(255, 0, 0));
                } else {
                    buff.set(i, frame_buffer::RGB::rgb(0, 255, 0));
                }
            }
        }

        // Copy frame buffer into texture
        buff.copy_to_texture(&mut texture);

        // Render texture
        canvas.copy(&texture, None, None);
        canvas.present();
     }
}
