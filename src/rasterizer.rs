use crate::frame_buffer::{FrameBuffer, RGB};
use crate::mesh::{Mesh, Vec3};

pub fn rasterize(mesh: &Mesh, buffer: &mut FrameBuffer, width: u32, height: u32) {
    for t in mesh.iter() {
        let p0 = t.p0;
        let p1 = t.p1;
        let p2 = t.p2;

        // Rasterise triangle
        let mut x = 0.0;
        let mut y = 0.0;
        let dx = 1.0 / (width as f32);
        let dy = 1.0 / (height as f32);

        // edge equations
        let n_1 = p0.cross(&p1);
        let n_2 = p2.cross(&p0);
        let n_3 = p1.cross(&p2);

        for i in 0..(width as usize) {
            for j in 0..(height as usize) {
                let p = Vec3::new(x, y, 1.0);

                // edge test
                if p.dot(&n_1) < 0.0 && p.dot(&n_3) < 0.0 && p.dot(&n_2) < 0.0 {
                    buffer.set(i * (width as usize) + j, t.color);
                }

                x += dx;
            }
            x = 0.0;
            y += dy;
        }
    }
}
