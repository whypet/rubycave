use std::{rc::Rc, sync::Arc};

use pollster::FutureExt;
use tracing::info;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::{config::Config, game::Game};

pub struct App {
    config: Rc<Config>,
    window: Option<Arc<Window>>,
    game: Option<Game<'static>>,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            config: Rc::new(config),
            window: None,
            game: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_transparent(true))
                .unwrap(),
        );
        let size = window.inner_size();

        self.window = Some(window.clone());

        self.game = Some(
            Game::new(window, self.config.clone(), size.width, size.height)
                .block_on()
                .expect("failed to create game"),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(game) = self.game.as_ref() else {
            return;
        };
        let Some(window) = self.window.as_ref() else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => {
                info!("exiting");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                game.get_state().resize(size.width, size.height);
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                let mut camera = game.get_camera();

                if let PhysicalKey::Code(code) = event.physical_key {
                    match code {
                        KeyCode::KeyW => {
                            camera.pos.x -= 0.1 * camera.ang.y.sin();
                            camera.pos.z -= 0.1 * camera.ang.y.cos();
                            camera.set_updated(true);
                        }
                        KeyCode::KeyA => {
                            let y = camera.ang.y + std::f32::consts::FRAC_PI_2;
                            camera.pos.x -= 0.1 * y.sin();
                            camera.pos.z -= 0.1 * y.cos();
                            camera.set_updated(true);
                        }
                        KeyCode::KeyS => {
                            camera.pos.x += 0.1 * camera.ang.y.sin();
                            camera.pos.z += 0.1 * camera.ang.y.cos();
                            camera.set_updated(true);
                        }
                        KeyCode::KeyD => {
                            let y = camera.ang.y + std::f32::consts::FRAC_PI_2;
                            camera.pos.x += 0.1 * y.sin();
                            camera.pos.z += 0.1 * y.cos();
                            camera.set_updated(true);
                        }
                        KeyCode::ArrowUp => {
                            camera.ang.x += 0.05;
                            camera.set_updated(true);
                        }
                        KeyCode::ArrowDown => {
                            camera.ang.x -= 0.05;
                            camera.set_updated(true);
                        }
                        KeyCode::ArrowLeft => {
                            camera.ang.y += 0.05;
                            camera.set_updated(true);
                        }
                        KeyCode::ArrowRight => {
                            camera.ang.y -= 0.05;
                            camera.set_updated(true);
                        }
                        _ => {}
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                game.render();
                window.request_redraw();
            }
            _ => (),
        }
    }
}
