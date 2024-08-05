use std::{io, rc::Rc, sync::Arc};

use tokio::runtime::{Builder, Runtime};
use tracing::{debug, error, info};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    event::{DeviceEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorGrabMode, Window, WindowId},
};

use crate::{config::Config, game::Game};

pub struct App<'a> {
    rt: Runtime,
    config: Rc<Config>,
    window: Option<Arc<Window>>,
    game: Option<Game<'a>>,
    focused: bool,
}

impl<'a> App<'a> {
    pub fn new(config: Config) -> io::Result<Self> {
        let rt = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        Ok(Self {
            rt,
            config: Rc::new(config),
            window: None,
            game: None,
            focused: false,
        })
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_transparent(true)
                        .with_blur(true),
                )
                .unwrap(),
        );
        let size = window.inner_size();

        self.window = Some(window.clone());

        self.game = Some(self.rt.block_on(async {
            Game::new(window, self.config.clone(), size.width, size.height)
                .await
                .unwrap()
        }));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(game) = self.game.as_mut() else {
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
                game.resize(size.width, size.height);
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    match code {
                        KeyCode::Escape => {
                            if event.state.is_pressed() {
                                self.focused ^= true;

                                let res = window.set_cursor_grab(if self.focused {
                                    CursorGrabMode::Locked
                                } else {
                                    CursorGrabMode::None
                                });
                                window.set_cursor_visible(!self.focused);

                                if let Err(error) = res {
                                    error!("failed to set cursor grab: {}", error);
                                } else if self.focused {
                                    debug!("grabbed cursor");
                                } else {
                                    debug!("let go of cursor");

                                    let size = window.inner_size();
                                    let res = window.inner_position();

                                    if let Ok(pos) = res {
                                        let _ = window.set_cursor_position(PhysicalPosition::new(
                                            pos.x + (size.width / 2) as i32,
                                            pos.y + (size.height / 2) as i32,
                                        ));
                                    }
                                }
                            }
                        }
                        _ => game.key(code, event.state.is_pressed()),
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                self.rt.block_on(game.update_async()).unwrap();
                game.update().unwrap();
                game.render().unwrap();
                window.request_redraw();
            }
            _ => (),
        }
    }

    fn device_event(
        &mut self,
        _: &ActiveEventLoop,
        _: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if !self.focused {
            return;
        }
        let Some(window) = self.window.as_ref() else {
            return;
        };

        if let DeviceEvent::MouseMotion { delta } = event {
            let Some(game) = self.game.as_mut() else {
                return;
            };

            game.mouse(delta, window.inner_size());
        }
    }
}
