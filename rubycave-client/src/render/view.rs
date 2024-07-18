use rubycave::glam::{Affine3A, EulerRot, Mat4, Quat, Vec3};

use crate::config::Config;

#[derive(Default)]
pub struct Camera {
    is_updated: bool,

    pub pos: Vec3,
    pub ang: Vec3,
}

impl Camera {
    pub fn new(pos: Vec3, ang: Vec3) -> Self {
        Self {
            is_updated: false,
            pos,
            ang,
        }
    }

    pub fn set_updated(&mut self, is_updated: bool) {
        self.is_updated = is_updated;
    }

    pub fn is_updated(&self) -> bool {
        self.is_updated
    }

    pub fn view(&self) -> Affine3A {
        Affine3A::look_to_rh(
            self.pos,
            Quat::from_euler(EulerRot::XYZ, self.ang.x, self.ang.y, self.ang.z)
                .mul_vec3(Vec3::NEG_Z),
            Vec3::Y,
        )
    }
}

pub fn perspective_rh(config: &Config, width: f32, height: f32) -> Mat4 {
    Mat4::perspective_rh(config.get_fov().to_radians(), width / height, 0.1, 100.0)
}
