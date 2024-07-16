use rubycave::space::{Orientation, Position};

use crate::math;

#[derive(Default)]
pub struct Camera {
    pub pos: Position,
    pub ang: Orientation,
}

impl Camera {
    #[rustfmt::skip]
    pub fn view_mat(&self) -> [f32; 16] {
        let sin_yaw = (-self.ang.yaw).sin();
        let cos_yaw = (-self.ang.yaw).cos();
        let sin_pitch = (-self.ang.pitch).sin();
        let cos_pitch = (-self.ang.pitch).cos();
        let sin_roll = (-self.ang.roll).sin();
        let cos_roll = (-self.ang.roll).cos();

        let x = [
            cos_yaw, -sin_yaw, 0.0,
            sin_yaw, cos_yaw, 0.0,
            0.0, 0.0, 1.0,
        ];

        let y = [
            cos_pitch, 0.0, sin_pitch,
            0.0, 1.0, 0.0,
            -sin_pitch, 0.0, cos_pitch
        ];

        let z = [
            1.0, 0.0, 0.0,
            0.0, cos_roll, -sin_roll,
            0.0, sin_roll, cos_roll
        ];

        let origin_mat = [
            1.0, 0.0, 0.0, -self.pos.x,
            0.0, 1.0, 0.0, -self.pos.y,
            0.0, 0.0, 1.0, -self.pos.z,
            0.0, 0.0, 0.0, 1.0
        ];

        math::mul_mat4(math::mat3_to_mat4(math::mul_mat3(math::mul_mat3(z, y), x)), origin_mat)
    }
}

#[rustfmt::skip]
pub fn ortho_proj(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) -> [f32; 16] {
    [
        2.0 / (right - left),
        0.0,
        0.0,
        0.0,

        0.0,
        2.0 / (top - bottom),
        0.0,
        0.0,

        0.0,
        0.0,
        -2.0 / (far - near),
        0.0,

        -(right + left) / (right - left),
        -(top + bottom) / (top - bottom),
        -(far + near) / (far - near),
        1.0,
    ]
}
