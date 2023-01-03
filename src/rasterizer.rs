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

        // TODO: Implement more efficient screen traversal (zigzag or tiled maybe?)
        for i in 0..(buffer.width()) {
            for j in 0..(buffer.heigh()) {
                let p = (x, y);

                // edge test
                if e0.eval(p) >= 0.0 && e1.eval(p) >= 0.0 && e2.eval(p) >= 0.0 {
                    buffer.set(i * buffer.width() + j, t.color);
                }

                x += dx;
            }
            x = 0.0;
            y += dy;
        }
    }
}
