use rubycave::glam::{Affine3A, Vec3};

#[derive(Default)]
pub struct Camera {
    pub pos: Vec3,
    pub ang: Vec3,
}

impl Camera {
    #[rustfmt::skip]
    pub fn view(&self) -> Affine3A {
        Affine3A::look_at_rh(self.pos, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0))
    }
}
