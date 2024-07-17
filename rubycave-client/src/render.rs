use std::cell::RefCell;

pub mod game;
pub mod triangle;
pub mod view;

pub trait Renderer {
    // fn new(state_ref: StateRef, resource_man: &'state ResourceManager) -> Self;
    fn render(&self);
}

pub struct State<'window> {
    #[allow(dead_code)]
    instance: wgpu::Instance,
    surface: wgpu::Surface<'window>,
    adapter: wgpu::Adapter,
    surface_config: RefCell<wgpu::SurfaceConfiguration>,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl<'window> State<'window> {
    pub async fn new(
        target: impl Into<wgpu::SurfaceTarget<'window>>,
        width: u32,
        height: u32,
    ) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

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

        let surface_config = surface.get_default_config(&adapter, width, height).unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .expect("failed to create device");

        surface.configure(&device, &surface_config);

        let surface_config = RefCell::new(surface_config);

        Self {
            instance,
            surface,
            adapter,
            surface_config,
            device,
            queue,
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        let mut config = self.surface_config.borrow_mut();

        config.width = width;
        config.height = height;

        self.surface.configure(&self.device, &config);
    }
}
