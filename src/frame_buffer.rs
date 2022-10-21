extern crate sdl2;
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
    heigt: usize,
    buff_size: usize,
    buffer: Vec<u8>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let buff_size = width * height * 3;
        FrameBuffer {
            width: width,
            heigt: height,
            buff_size: buff_size,
            buffer: vec![0; buff_size],
        }
    }

    pub fn size(&self) -> usize {
        self.width * self.heigt
    }

    // Todo: overload array operation instead
    pub fn set(&mut self, index: usize, color: RGB) {
        let i = index * 3;
        self.buffer[i] = color.r;
        self.buffer[i + 1] = color.g;
        self.buffer[i + 2] = color.b;
    }

    pub fn copy_to_texture(&self, texture: &mut sdl2::render::Texture) {
        texture.with_lock(None, |buff: &mut [u8], _: usize| {
            buff[..self.buff_size].copy_from_slice(&self.buffer)
        });
    }
}
