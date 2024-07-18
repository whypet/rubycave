use std::cell::RefCell;

pub mod game;
pub mod test;
pub mod view;
pub mod world;

pub trait Renderer {
    fn render(&mut self);
}

pub trait SizedSurface {
    fn get_size(&self) -> (u32, u32);
    #[allow(dead_code)]
    fn get_width(&self) -> u32;
    #[allow(dead_code)]
    fn get_height(&self) -> u32;
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

        let mut surface_config = surface.get_default_config(&adapter, width, height).unwrap();
        surface_config.alpha_mode = wgpu::CompositeAlphaMode::PreMultiplied;

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

        surface.configure(&device, &surface_config);

        let surface_config = RefCell::new(surface_config);
        let _ = surface_config.get_size();

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

impl<'window> SizedSurface for RefCell<wgpu::SurfaceConfiguration> {
    fn get_size(&self) -> (u32, u32) {
        let surface_config = self.borrow();
        (surface_config.width, surface_config.height)
    }

    fn get_width(&self) -> u32 {
        self.borrow().width
    }

    fn get_height(&self) -> u32 {
        self.borrow().height
    }
}
