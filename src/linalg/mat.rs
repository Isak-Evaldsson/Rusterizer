use super::Vec4;
use std::ops;

pub struct Mat4x4 {
    elements: [[f32; 4]; 4],
}

impl Mat4x4 {
    pub fn diag(n: f32) -> Self {
        Mat4x4 {
            elements: [
                [n, 0.0, 0.0, 0.0],
                [0.0, n, 0.0, 0.0],
                [0.0, 0.0, n, 0.0],
                [0.0, 0.0, 0.0, n],
            ],
        }
    }

    pub fn fill(n: f32) -> Self {
        Mat4x4 {
            elements: [[n, n, n, n], [n, n, n, n], [n, n, n, n], [n, n, n, n]],
        }
    }

    pub fn empty() -> Self {
        Self::fill(0.0)
    }

    pub fn identity() -> Self {
        Self::diag(1.0)
    }
}

/*
    Good looking matrix (row, col) accessing
*/
impl ops::Index<(usize, usize)> for Mat4x4 {
    type Output = f32;

    fn index(&self, i: (usize, usize)) -> &Self::Output {
        &self.elements[i.0][i.1]
    }
}

impl ops::IndexMut<(usize, usize)> for Mat4x4 {
    fn index_mut(&mut self, i: (usize, usize)) -> &mut Self::Output {
        &mut self.elements[i.0][i.1]
    }
}

/*
    Arithmetic operator overloading
*/

// Self * Vec4
impl ops::Mul<Vec4> for Mat4x4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        let x = rhs.x * self.elements[0][0]
            + rhs.y * self.elements[0][1]
            + rhs.z * self.elements[0][2]
            + rhs.w * self.elements[0][3];

        let y = rhs.x * self.elements[1][0]
            + rhs.y * self.elements[1][1]
            + rhs.z * self.elements[1][2]
            + rhs.w * self.elements[1][3];

        let z = rhs.x * self.elements[2][0]
            + rhs.y * self.elements[2][1]
            + rhs.z * self.elements[2][2]
            + rhs.w * self.elements[2][3];

        let w = rhs.x * self.elements[3][0]
            + rhs.y * self.elements[3][1]
            + rhs.z * self.elements[3][2]
            + rhs.w * self.elements[3][3];

        Vec4::new(x, y, z, w)
    }
}

// Self * Mat4x4
impl ops::Mul<Mat4x4> for Mat4x4 {
    type Output = Mat4x4;

    fn mul(self, rhs: Mat4x4) -> Self::Output {
        let mut result = Mat4x4::empty();

        for i in 0..3 {
            for k in 0..3 {
                for j in 0..3 {
                    result[(i, j)] += self[(i, k)] * rhs[(k, j)];
                }
            }
        }

        result
    }
}
