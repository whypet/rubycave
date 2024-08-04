use std::{ops::DerefMut, rc::Rc};

use rubycave::glam::Vec3;
use winit::{dpi::PhysicalSize, keyboard::KeyCode};

use crate::{config::Config, entity::Entity};

pub struct InputMovement {
    config: Rc<Config>,
    wasdqe: [bool; 6],
    rot: Vec3,
}

impl InputMovement {
    pub fn new(config: Rc<Config>) -> Self {
        Self {
            config,
            wasdqe: Default::default(),
            rot: Vec3::default(),
        }
    }

    pub fn key(&mut self, key: KeyCode, down: bool) {
        match key {
            KeyCode::KeyW => {
                self.wasdqe[0] = down;
            }
            KeyCode::KeyA => {
                self.wasdqe[1] = down;
            }
            KeyCode::KeyS => {
                self.wasdqe[2] = down;
            }
            KeyCode::KeyD => {
                self.wasdqe[3] = down;
            }
            KeyCode::KeyQ | KeyCode::Space => {
                self.wasdqe[4] = down;
            }
            KeyCode::KeyE | KeyCode::ShiftLeft => {
                self.wasdqe[5] = down;
            }
            _ => {}
        }
    }

    pub fn mouse(&mut self, delta: (f64, f64), window_size: PhysicalSize<u32>) {
        self.rot += Vec3::new(
            delta.0 as f32 / window_size.width as f32,
            delta.1 as f32 / window_size.height as f32,
            0.0,
        );
    }

    pub fn update(&mut self, mut entity: impl DerefMut<Target = impl Entity>) {
        let wasd = &self.wasdqe[0..4];
        let qe = &self.wasdqe[4..6];

        let motion_mul = Vec3::ONE * 0.1;

        if wasd.iter().any(|x| *x) {
            let mut wa_angle = entity.get_head();
            let mut sd_angle = wa_angle;

            if wasd[1] {
                wa_angle.x += if wasd[0] {
                    std::f32::consts::FRAC_PI_4
                } else {
                    std::f32::consts::FRAC_PI_2
                };
            }
            if wasd[3] {
                sd_angle.x += if wasd[2] {
                    std::f32::consts::FRAC_PI_4
                } else {
                    std::f32::consts::FRAC_PI_2
                };
            }

            let wa_motion = Vec3::new(wa_angle.x.sin(), 0.0, wa_angle.x.cos())
                * if wasd[0] || wasd[1] {
                    -motion_mul
                } else {
                    Vec3::ZERO
                };
            let sd_motion = Vec3::new(sd_angle.x.sin(), 0.0, sd_angle.x.cos())
                * if wasd[2] || wasd[3] {
                    motion_mul
                } else {
                    Vec3::ZERO
                };

            entity.move_by(wa_motion + sd_motion);
        }

        if qe[0] != qe[1] {
            entity.move_by(Vec3::Y * (qe[0] as i32 - qe[1] as i32) as f32 * motion_mul);
        }

        if self.rot != Vec3::ZERO {
            entity.move_head(-self.rot * self.config.sensitivity);
            self.rot = Vec3::ZERO;
        }
    }
}
