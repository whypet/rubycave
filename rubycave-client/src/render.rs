use std::borrow::Borrow;

use crate::resource::ResourceManager;

pub mod game;
pub mod triangle;

pub trait Renderer<'state, StateRef: Borrow<State<'state>>> {
    fn new(state_ref: StateRef, resource_man: &'state ResourceManager) -> Self;
    fn render(&self);
}

pub struct State<'window> {
    #[allow(dead_code)]
    instance: wgpu::Instance,
    surface: wgpu::Surface<'window>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl<'window> State<'window> {
    pub async fn new(target: impl Into<wgpu::SurfaceTarget<'window>>) -> Self {
        let instance = wgpu::Instance::default();

        let surface = instance
            .create_surface(target)
            .expect("failed to create surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                },
                None,
            )
            .await
            .expect("failed to create device");

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        let config = self
            .surface
            .get_default_config(&self.adapter, width, height)
            .unwrap();
        self.surface.configure(&self.device, &config);
    }
}
