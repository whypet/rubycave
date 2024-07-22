use std::{
    cell::{Ref, RefCell},
    env, io,
    rc::Rc,
    time::Instant,
};

use crate::{
    config::Config,
    entity::{Entity, Player},
    render::{self, game::GameRenderer, view::Camera, Renderer, State},
    resource::{self, ResourceManager},
};
use rubycave::glam::Vec3;
use winit::{dpi::PhysicalSize, keyboard::KeyCode};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("render state error")]
    RenderState(#[from] render::StateError),
    #[error("resource error")]
    Resource(#[from] resource::Error),
}

pub struct Game<'a> {
    state: Rc<State<'a>>,
    config: Rc<Config>,
    player: Rc<RefCell<Player>>,
    camera: Rc<RefCell<Camera>>,
    renderer: RefCell<GameRenderer<'a>>,
    last_update: Instant,
    wasdqe: [bool; 6],
    rot: Vec3,
}

impl<'a> Game<'a> {
    pub async fn new(
        target: impl Into<wgpu::SurfaceTarget<'a>>,
        config: Rc<Config>,
        width: u32,
        height: u32,
    ) -> Result<Self, Error> {
        let state = Rc::new(State::new(target, width, height).await?);
        let resource_man = Rc::new(ResourceManager::new(
            env::current_exe()?.parent().unwrap().join("res").as_path(),
        ));
        let player = Rc::new(RefCell::new(Player::new(Vec3::ZERO)));
        let camera = Rc::new(RefCell::new(Camera::new(
            Vec3::new(0.0, 0.0, 3.0),
            Vec3::ZERO,
        )));

        Ok(Self {
            state: state.clone(),
            config: config.clone(),
            player,
            camera: camera.clone(),
            renderer: RefCell::new(GameRenderer::new(state, config, resource_man, camera)?),
            last_update: Instant::now(),
            wasdqe: Default::default(),
            rot: Vec3::ZERO,
        })
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

    pub fn mouse_delta(&mut self, delta: (f64, f64), window_size: PhysicalSize<u32>) {
        self.rot += Vec3::new(
            delta.0 as f32 / window_size.width as f32,
            delta.1 as f32 / window_size.height as f32,
            0.0,
        );
    }

    pub fn update(&mut self) {
        let mut player = self.player.borrow_mut();
        let wasd = &self.wasdqe[0..4];
        let qe = &self.wasdqe[4..6];

        let motion_mul = Vec3::ONE * 0.1;

        if wasd.iter().any(|x| *x) {
            let mut wa_angle = self.get_camera().ang;
            let mut sd_angle = self.get_camera().ang;

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

            player.move_by(wa_motion + sd_motion);
        }

        if qe[0] != qe[1] {
            player.move_by(Vec3::Y * (qe[0] as i32 - qe[1] as i32) as f32 * motion_mul);
        }

        if self.rot != Vec3::ZERO {
            player.move_head(-self.rot * self.config.sensitivity);
            self.rot = Vec3::ZERO;
        }

        player.update(self.last_update.elapsed().as_secs_f32());

        self.last_update = Instant::now();

        self.renderer.borrow_mut().update();
    }

    pub fn render(&self) -> Result<(), render::StateError> {
        {
            let mut camera = self.camera.borrow_mut();
            let player = self.player.borrow();
            let pos = player.get_position();
            let ang = player.get_head();

            if camera.pos != pos || camera.ang != ang {
                camera.pos = pos;
                camera.ang = ang;
                camera.set_updated(true);
            } else {
                camera.set_updated(false);
            }
        }

        let frame = self.state.get_frame()?;
        let view = &frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut renderer = self.renderer.borrow_mut();

        self.state.submit(renderer.render(view));
        frame.present();

        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.state.resize(width, height);
        self.renderer.borrow_mut().resize(width, height);
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn get_camera(&self) -> Ref<Camera> {
        self.camera.borrow()
    }
}
