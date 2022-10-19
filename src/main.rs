mod vector;

// sdl
extern crate sdl2;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture};
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::time::Duration;


// Screen cosntants
const WIDTH: u32= 800;
const HEIGHT: u32 = 600;
const PIXEL_PITCH: u32 = 4;

fn main() {
    // Triangle
    let p0 = vector::vec3(0.0, 0.0, 0.0);
    let p0 = vector::vec3(0.0, 0.0, 1.0);
    let p0 = vector::vec3(0.0, 1.0, 1.0);


    // Frame buffer
    let buff_size = (WIDTH * HEIGHT * PIXEL_PITCH) as usize;
    let mut frame_buffer: Vec<u8> = vec![0; buff_size];

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
    let mut texture = texture_creator.create_texture_streaming(None, WIDTH, HEIGHT)
        .expect("Could not create texture");

    let mut event_pump = sdl_context.event_pump().unwrap();

    // drawing loop
    'main: loop {
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        canvas.clear();


        for event in event_pump.poll_iter() {
            match event {
                // quit on window exit click
                Event::Window { timestamp: _, window_id: _, win_event: WindowEvent::Close } => {
                    break 'main;
                },
                _ => {}
            }
        }


        // fill with red and gree
        for i in 0..WIDTH*HEIGHT {
            frame_buffer[i as usize] = 255;
            frame_buffer[(i + 3) as usize] = 255;
        }

        // copy frame buffer into texture
        texture.with_lock(None, | buff: &mut[u8], _: usize| { buff[..buff_size].copy_from_slice(&frame_buffer)});

        // render texture
        canvas.copy(&texture, None, None);
        canvas.present();
    }
}
