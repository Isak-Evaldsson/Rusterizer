use crate::frame_buffer::RGB;
pub use crate::linalg::Vec3;
use std::slice::Iter;

pub struct Triangle {
    pub p0: Vec3,
    pub p1: Vec3,
    pub p2: Vec3,
    pub color: RGB,
}

pub struct Mesh {
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(list: Vec<Triangle>) -> Self {
        Mesh { triangles: list }
    }

    pub fn iter(&self) -> Iter<Triangle> {
        self.triangles.iter()
    }
}

impl Triangle {
    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, color: RGB) -> Self {
        Triangle {
            p0: p0,
            p1: p1,
            p2: p2,
            color: color,
        }
    }
}
