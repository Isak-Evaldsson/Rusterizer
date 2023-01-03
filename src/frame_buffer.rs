extern crate sdl2;

#[derive(Copy, Clone)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        RGB { r: r, g: g, b: b }
    }
}

pub struct FrameBuffer {
    width: usize,
    heigh: usize,
    buff_size: usize,
    color_buffer: Vec<u8>,
    depth_buffer: Vec<f32>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let n_elements = width * height;
        let buff_size = n_elements * 3;

        FrameBuffer {
            width: width,
            heigh: height,
            buff_size: buff_size,
            color_buffer: vec![0; buff_size],
            depth_buffer: vec![0.0; n_elements],
        }
    }

    // Todo: overload array operation instead
    pub fn set(&mut self, index: usize, color: RGB) {
        let i = index * 3;
        self.color_buffer[i] = color.r;
        self.color_buffer[i + 1] = color.g;
        self.color_buffer[i + 2] = color.b;
    }

    pub fn test_and_set(&mut self, index: usize, color: RGB, depth: f32) {
        let i = index * 3;

        if depth > self.depth_buffer[index] {
            self.depth_buffer[index] = depth;

            self.color_buffer[i] = color.r;
            self.color_buffer[i + 1] = color.g;
            self.color_buffer[i + 2] = color.b;
        }
    }

    pub fn copy_to_texture(&self, texture: &mut sdl2::render::Texture) {
        texture
            .with_lock(None, |buff: &mut [u8], _: usize| {
                buff[..self.buff_size].copy_from_slice(&self.color_buffer)
            })
            .unwrap();
    }

    pub fn heigh(&self) -> usize {
        self.heigh
    }

    pub fn width(&self) -> usize {
        self.width
    }
}
