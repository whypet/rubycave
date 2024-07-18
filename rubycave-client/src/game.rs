use std::{
    cell::{RefCell, RefMut},
    env, io,
    rc::Rc,
};

use rubycave::glam::Vec3;

use crate::{
    config::Config,
    render::{game::GameRenderer, view::Camera, Renderer, State},
    resource::ResourceManager,
};

pub struct Game<'a> {
    state: Rc<State<'a>>,
    camera: Rc<RefCell<Camera>>,
    renderer: GameRenderer<'a>,
}

impl<'a> Game<'a> {
    pub async fn new(
        target: impl Into<wgpu::SurfaceTarget<'a>>,
        config: Rc<Config>,
        width: u32,
        height: u32,
    ) -> io::Result<Self> {
        let state = Rc::new(State::new(target, width, height).await);
        let resource_man = Rc::new(ResourceManager::new(env::current_exe()?.parent().unwrap()));
        let camera = Rc::new(RefCell::new(Camera::new(
            Vec3::new(0.0, 0.0, 3.0),
            Vec3::default(),
        )));

        Ok(Self {
            state: state.clone(),
            camera: camera.clone(),
            renderer: GameRenderer::new(state, config, resource_man, camera),
        })
    }

    pub fn render(&self) {
        self.renderer.render();

        self.get_camera().set_updated(false);
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn get_camera(&self) -> RefMut<Camera> {
        self.camera.borrow_mut()
    }
}
