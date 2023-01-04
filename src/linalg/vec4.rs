use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

// Vector functions
impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 { x, y, z, w }
    }

    pub fn dot(&self, v: &Vec4) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }
}

/*
    Operator Overloading
*/

// Vec4 + Vec4
impl ops::Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, _rhs: Vec4) -> Vec4 {
        Self::new(
            self.x + _rhs.x,
            self.y + _rhs.y,
            self.z + _rhs.z,
            self.w + _rhs.w,
        )
    }
}

// Vec4 - Vec4
impl ops::Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, _rhs: Vec4) -> Vec4 {
        Self::new(
            self.x - _rhs.x,
            self.y - _rhs.y,
            self.z - _rhs.z,
            self.w - _rhs.w,
        )
    }
}

// Vec4 + f32
impl ops::Add<f32> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: f32) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs, self.z + rhs, self.w + rhs)
    }
}

// f32 + Vec4
impl ops::Add<Vec4> for f32 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        Vec4::new(self + rhs.x, self + rhs.y, self + rhs.z, self + rhs.w)
    }
}

// f32 * Vec4
impl ops::Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

// Vec4 * f32
impl ops::Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}
