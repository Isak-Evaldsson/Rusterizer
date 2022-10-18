use std::ops;

    #[derive(Debug)]
pub struct Vec3 {
    x:f32,
    y:f32,
    z:f32
}

// Constructor
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x: x, y: y, z: z }
}

// Vector additon
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        vec3(self.x + _rhs.x, self.y + _rhs.y, self.z + self.z)
    }
}

// Vector scalar additon
impl ops::Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        vec3(self + rhs.x, self + rhs.y, self + rhs.z)
    }
}

// Vector scalar multiplication
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        vec3(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}
