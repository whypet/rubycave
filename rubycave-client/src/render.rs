use std::borrow::Borrow;

use wgpu::SurfaceTarget;

pub mod game;
pub mod triangle;

pub trait Renderer<'window, StateRef: Borrow<State<'window>>> {
    fn new(state: StateRef) -> Self;
    fn render(&self);
}

#[allow(dead_code)]
pub struct State<'window> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'window>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl<'window> State<'window> {
    pub async fn new(target: impl Into<SurfaceTarget<'window>>) -> Self {
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
}
