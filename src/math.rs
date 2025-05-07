use std::ops::{Add, Sub, Mul};
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const N: usize> {
    pub data: [[f32; N]; N]
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[macro_export]
macro_rules! identity_matrix {
    ($n:expr) => {{
        let mut data = [[0.0f32; $n]; $n];
        let mut i = 0;
        while i < $n {
            data[i][i] = 1.0;
            i += 1;
        }
        Matrix::<$n> { data }
    }};
}

impl <const N: usize> Add for Matrix<N> {
    type Output = Matrix<N>;

    fn add(self, other: Matrix<N>) -> Matrix<N> {
        let mut result = [[0.0f32; N]; N];


        for i in 0..N {
            for j in 0..N {
                result[i][j] = self.data[i][j] + other.data[i][j];
            }
        }

        Matrix::<N> { data: result }
    }
}

impl <const N: usize> Mul for Matrix<N> {
    type Output = Matrix<N>;

    fn mul(self, other: Matrix<N>) -> Matrix<N> {
        let mut result = [[0.0f32; N]; N];

        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    result[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }

        Matrix::<N> { data: result }
    }
}

impl Vector3 {
    pub fn normalize(self) -> Self {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vector3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn dot(self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Matrix<4> {
    fn translation_matrix(translation: Vector3) -> Matrix<4> {
        let mut mat = identity_matrix!(4);
        mat.data[0][3] = translation.x;
        mat.data[1][3] = translation.y;
        mat.data[2][3] = translation.z;
        mat
    }

    fn scale_matrix(scale: Vector3) -> Matrix<4> {
        let mut mat = identity_matrix!(4);
        mat.data[0][0] = scale.x;
        mat.data[1][1] = scale.y;
        mat.data[2][2] = scale.z;
        mat
    }

    fn rotation_x_matrix(angle: f32) -> Matrix<4> {
        let mut mat = identity_matrix!(4);
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        mat.data[0][0] = cos_angle;
        mat.data[0][1] = -sin_angle;
        mat.data[1][0] = sin_angle;
        mat.data[1][1] = cos_angle;
        mat
    }

    fn rotation_y_matrix(angle: f32) -> Matrix<4> {
        let mut mat = identity_matrix!(4);
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        mat.data[0][0] = cos_angle;
        mat.data[0][2] = sin_angle;
        mat.data[2][0] = -sin_angle;
        mat.data[2][2] = cos_angle;
        mat
    }

    fn rotation_z_matrix(angle: f32) -> Matrix<4> {
        let mut mat = identity_matrix!(4);
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        mat.data[0][0] = cos_angle;
        mat.data[0][1] = -sin_angle;
        mat.data[1][0] = sin_angle;
        mat.data[1][1] = cos_angle;
        mat
    }

    pub fn transform_matrix(translation: Vector3, rotation: Vector3, scale: Vector3) -> Matrix<4> {
        let translation_mat = Self::translation_matrix(translation);
        let scale_mat = Self::scale_matrix(scale);
        let rot_x_mat = Self::rotation_x_matrix(rotation.x);
        let rot_y_mat = Self::rotation_y_matrix(rotation.y);
        let rot_z_mat = Self::rotation_z_matrix(rotation.z);

        translation_mat * scale_mat * rot_z_mat * rot_x_mat * rot_y_mat
    }

    pub fn perspective_matrix(fov: f32, aspect: f32, near: f32, far: f32) -> Matrix<4> {
        let tan_half_fov = (fov / 2.0).tan();


        let mut mat = identity_matrix!(4);

        mat.data[0][0] = 1.0 / (tan_half_fov * aspect);
        mat.data[1][1] = 1.0 / tan_half_fov;
        mat.data[2][2] = (far + near) / (near - far);
        mat.data[2][3] = (2.0 * far * near) / (near - far);
        mat.data[3][2] = -1.0;

        mat
    }
}
