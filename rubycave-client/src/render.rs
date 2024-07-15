use std::borrow::Borrow;

use ouroboros::self_referencing;
use triangle::TriangleRenderer;
use wgpu::SurfaceTarget;

pub mod triangle;

pub trait Renderer {
    fn render(&mut self);
}

trait InternalRenderer<'window, StateRef: Borrow<State<'window>>>: Renderer {
    fn new(state: StateRef) -> Self;
}

struct State<'window> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'window>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

#[self_referencing]
struct InnerGameRenderer<'window> {
    state: State<'window>,
    #[borrows(state)]
    #[not_covariant]
    triangle_renderer: TriangleRenderer<'this, &'this State<'this>>,
}

pub struct GameRenderer<'window> {
    inner: InnerGameRenderer<'window>,
}

impl<'a> GameRenderer<'a> {
    pub async fn new(target: impl Into<SurfaceTarget<'a>>) -> Self {
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

        let state = State {
            instance,
            surface,
            adapter,
            device,
            queue,
        };

        Self {
            inner: InnerGameRenderer::new(state, |s| TriangleRenderer::new(s)),
        }
    }
}

impl<'window> Renderer for GameRenderer<'window> {
    fn render(&mut self) {}
}
