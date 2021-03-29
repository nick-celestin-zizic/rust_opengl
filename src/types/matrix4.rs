//use crate::types::*;
use std::ops::{Add, AddAssign,
               Mul, MulAssign};
use crate::util::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tests() {
        let left  = Matrix4([
            [3.0, 0.0, 0.0, 0.0],
            [0.0, 3.0, 0.0, 0.0],
            [0.0, 0.0, 3.0, 0.0],
            [1.0, 0.0, 0.0, 3.0]
        ]);
        let right = Matrix4([
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 9.0, 2.0, 0.0],
            [0.0, 0.0, 0.0, 2.0]
        ]);

        assert_eq!(left * right, Matrix4([
            [6.0, 0.0, 0.0, 0.0],
            [0.0, 6.0, 0.0, 0.0],
            [0.0, 27.0, 6.0, 0.0],
            [2.0, 0.0, 0.0, 6.0]
        ]))
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Matrix4(pub [[f32; 4]; 4]);

impl Add for Matrix4 {
    type Output = Self;

    fn add(self, _other: Self) -> Self {
        todo!();
    }
}

impl AddAssign for Matrix4 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Mul for Matrix4 {
    type Output = Self;

    //TODO this is straight up wrong im a retard
    fn mul(self, other: Self) -> Self {
        let mut out = Matrix4::default();
        for i in 0..4 { for j in 0..4 {
            let mut num = 0.0;
            for k in 0..4 {
                num += self.0[i][k] * other.0[k][j];
            }
            out.0[i][j] = num;
        }}
        out
    }
}

impl MulAssign for Matrix4 {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

//#[allow(unused_variables)]
impl Matrix4 {
    pub fn rotate_about_x(&mut self, angle: f32) {
        let mut rotation = Matrix4::identity();
        let sine   = angle.sin();
        let cosine = angle.cos();
        
        rotation.0[1][1] =  cosine;
        rotation.0[1][2] = -sine;
        rotation.0[2][1] =  sine;
        rotation.0[2][2] =  cosine;

        *self *= rotation;
    }
    pub fn rotate_about_y(&mut self, angle: f32) {
        let mut rotation = Matrix4::identity();
        let sine   = angle.sin();
        let cosine = angle.cos();
        
        rotation.0[0][0] =  cosine;
        rotation.0[2][0] =  sine;
        rotation.0[0][2] = -sine;
        rotation.0[2][2] =  cosine;

        *self *= rotation;
    }
    pub fn rotate_about_z(&mut self, angle: f32) {
        let mut rotation = Matrix4::identity();
        let sine   = angle.sin();
        let cosine = angle.cos();

        rotation.0[0][0] =  cosine;
        rotation.0[0][1] = -sine;
        rotation.0[1][0] =  sine;
        rotation.0[1][1] =  cosine;
        
        *self *= rotation;
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        let mut scale = Self::identity();
        
        scale.0[0][0] = x;
        scale.0[1][1] = y;
        scale.0[2][2] = z;

        *self *= scale;
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        let mut translation = Matrix4::identity();
        
        translation.0[3][0] = x;
        translation.0[3][1] = y;
        translation.0[3][2] = z;

        *self *= translation;
    }

    pub fn projection(fovy:       f32, aspect_ratio: f32,
                      near_plane: f32, far_plane:    f32) -> Self {
        let mut out = Matrix4::default();
        let y_scale = 1.0 / f32::tan(deg_to_rad(fovy / 2.0));
        let x_scale = y_scale / aspect_ratio;
        let frustum_length = far_plane - near_plane;

        out.0[0][0] = x_scale;
        out.0[1][1] = y_scale;
        out.0[2][2] = -((far_plane + near_plane) / frustum_length);
        out.0[2][3] = -((2.0 * near_plane * far_plane) / frustum_length);
        out.0[3][2] = -1.0;

        out
    }
    pub fn identity() -> Self {
        Matrix4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ])
    }
}
