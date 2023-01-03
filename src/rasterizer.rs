use crate::frame_buffer::FrameBuffer;
use crate::mesh::{Mesh, Vec3};

#[derive(Debug)]
struct EdgeFunc {
    a: f32,
    b: f32,
    p0: Vec3,
}

impl EdgeFunc {
    fn new(p0: Vec3, p1: Vec3) -> Self {
        let a = -(p1.y - p0.y);
        let b = p1.x - p0.x;

        EdgeFunc { a, b, p0 }
    }

    // computes e(u) = n dot (u−p0)
    fn eval(&self, u: (f32, f32)) -> f32 {
        // 2d vector p0
        let p0_x = self.p0.x;
        let p0_y = self.p0.y;

        self.a * (u.0 - p0_x) + self.b * (u.1 - p0_y)
    }
}

pub fn rasterize(mesh: &Mesh, buffer: &mut FrameBuffer) {
    for t in mesh.iter() {
        let p0 = t.p0;
        let p1 = t.p1;
        let p2 = t.p2;

        // Rasterize triangle
        let mut x = 0.0;
        let mut y = 0.0;
        let dx = 1.0 / (buffer.width() as f32);
        let dy = 1.0 / (buffer.heigh() as f32);

        // Defining edge equations
        let e0 = EdgeFunc::new(p2, p1);
        let e1 = EdgeFunc::new(p0, p2);
        let e2 = EdgeFunc::new(p1, p0);

        // 1/(2 * triangle area), used for barycentric coords
        let half_recip_inv = 1.0 / (p1 - p0).cross(&(p2 - p0)).length();

        // TODO: Implement more efficient screen traversal (zigzag or tiled maybe?)
        for i in 0..(buffer.width()) {
            for j in 0..(buffer.heigh()) {
                let p = (x, y);

                // edge equation computation
                let e0_val = e0.eval(p);
                let e1_val = e1.eval(p);
                let e2_val = e2.eval(p);

                // edge test
                if e0_val >= 0.0 && e1_val >= 0.0 && e2_val >= 0.0 {
                    // barycentric coords
                    let u = e1_val * half_recip_inv;
                    let v = e2_val * half_recip_inv;

                    // interpolate depth from barycentrics
                    let depth = p0.z + u * (p1.z - p0.z) + v * (p2.z - p0.z);

                    buffer.test_and_set(i * buffer.width() + j, t.color, depth);
                }

                x += dx;
            }
            x = 0.0;
            y += dy;
        }
    }
}
