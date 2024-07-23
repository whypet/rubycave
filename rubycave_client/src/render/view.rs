use rubycave::glam::{Affine3A, EulerRot, Mat4, Quat, Vec3};

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
        let quat = Quat::from_euler(EulerRot::YXZ, self.ang.x, self.ang.y, self.ang.z);
        Affine3A::look_to_rh(self.pos, quat * Vec3::NEG_Z, quat * Vec3::Y)
    }
}

pub fn perspective_rh(fov: f32, width: u32, height: u32) -> Mat4 {
    Mat4::perspective_rh(fov, width as f32 / height as f32, 0.1, 100.0)
}
