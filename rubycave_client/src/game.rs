use std::{
    cell::{Ref, RefCell},
    env, io,
    rc::Rc,
    time::Instant,
};

use crate::{
    config::Config,
    entity::{Entity, Player},
    math::FastPrng,
    render::{self, game::GameRenderer, view::Camera, Renderer, State},
    resource::ResourceManager,
    rpc::{self, tcp::TcpClient, Client},
};
use input::InputMovement;
use rubycave::{
    epoch,
    glam::Vec3,
    protocol::{client, server, Packet},
    RangeIterator, KEEP_ALIVE_INTERVAL, TICK_RATE,
};
use tracing::{error, info};
use winit::{dpi::PhysicalSize, keyboard::KeyCode};

pub mod input;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("rpc error")]
    Rpc(#[from] rpc::Error),
    #[error("task join error")]
    Join(#[from] tokio::task::JoinError),
    #[error("render error")]
    Render(#[from] render::Error),
}

pub struct Game<'a> {
    game_rng: FastPrng<u32>,
    client: Option<TcpClient>,
    config: Rc<Config>,
    input: InputMovement,
    player: Rc<RefCell<Player>>,
    state: Rc<State<'a>>,
    camera: Rc<RefCell<Camera>>,
    renderer: RefCell<GameRenderer<'a>>,
    last_update: Instant,
    last_tick: Instant,
    last_keep_alive: Instant,
}

impl<'a> Game<'a> {
    pub async fn new(
        target: impl Into<wgpu::SurfaceTarget<'a>>,
        config: Rc<Config>,
        width: u32,
        height: u32,
    ) -> Result<Self, Error> {
        let mut game_rng = FastPrng::<u32> {
            state: epoch().as_millis() as u32,
        };
        let username = format!("Player{:0>4}", game_rng.next_in(0..=9999));

        let mut client = TcpClient::new("127.0.0.1:1616").await?;

        let client = if !client.start().await {
            error!("couldn't start rpc client");
            None
        } else {
            client.shake(&username).await?;
            Some(client)
        };

        let input = InputMovement::new(config.clone());
        let player = Rc::new(RefCell::new(Player::new(&username, Vec3::ZERO)));
        let state = Rc::new(State::new(target, width, height).await?);
        let resource_man = Rc::new(ResourceManager::new(
            env::current_exe()?.parent().unwrap().join("res").as_path(),
        ));
        let camera = Rc::new(RefCell::new(Camera::new(
            Vec3::new(0.0, 0.0, 3.0),
            Vec3::ZERO,
        )));

        let last = Instant::now();

        Ok(Self {
            game_rng,
            client,
            config: config.clone(),
            input,
            player,
            state: state.clone(),
            camera: camera.clone(),
            renderer: RefCell::new(GameRenderer::new(state, config, resource_man, camera)?),
            last_update: last,
            last_tick: last,
            last_keep_alive: last,
        })
    }

    pub fn key(&mut self, key: KeyCode, down: bool) {
        self.input.key(key, down)
    }

    pub fn mouse(&mut self, delta: (f64, f64), window_size: PhysicalSize<u32>) {
        self.input.mouse(delta, window_size)
    }

    pub async fn update_async(&mut self) -> Result<(), Error> {
        let Some(client) = &mut self.client else {
            return Ok(());
        };

        match client.poll()? {
            Some(Packet::Server(server::Packet::Kick { reason })) => {
                info!("kicked for: {:?}", reason);

                client.stop();
                self.client = None;

                return Ok(());
            }
            Some(Packet::Server(server::Packet::Teleport {
                x,
                y,
                z,
                yaw,
                pitch,
            })) => {
                info!("teleported to: {x:.1},{y:.1},{z:.1} {yaw:.1},{pitch:.1}");

                let mut player = self.player.borrow_mut();

                player.teleport(Vec3::new(x, y, z));
                player.set_head(Vec3::new(yaw, pitch, 0.0));
            }
            _ => {}
        }

        if self.last_tick.elapsed().as_nanos() as u64 >= 1_000_000_000 / (TICK_RATE as u64) {
            // info!("tick elapsed");
            self.last_tick = Instant::now();
        }

        if self.last_keep_alive.elapsed().as_millis() as u32 >= KEEP_ALIVE_INTERVAL {
            client
                .send(client::Packet::KeepAlive {
                    epoch: epoch().as_millis() as u64,
                })
                .await?;

            self.last_keep_alive = Instant::now();
        }

        Ok(())
    }

    pub fn update(&mut self) -> Result<(), Error> {
        {
            self.input.update(self.player.borrow_mut());
        }

        {
            self.player
                .borrow_mut()
                .update(self.last_update.elapsed().as_secs_f32());
        }

        self.last_update = Instant::now();

        self.renderer.borrow_mut().update();

        Ok(())
    }

    pub fn render(&self) -> Result<(), render::Error> {
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
