use std::cell::RefCell;

pub mod game;
pub mod test;
pub mod view;
pub mod world;

pub trait Renderer {
    fn update(&mut self);
    fn render<'p, 'a: 'p>(&'a mut self, frame_view: &wgpu::TextureView) -> wgpu::CommandBuffer;
    fn resize(&mut self, width: u32, height: u32);
}

pub trait SizedSurface {
    fn get_size(&self) -> (u32, u32);
    #[allow(dead_code)]
    fn get_width(&self) -> u32;
    #[allow(dead_code)]
    fn get_height(&self) -> u32;
}

pub struct State<'w> {
    #[allow(dead_code)]
    instance: wgpu::Instance,
    surface: wgpu::Surface<'w>,
    adapter: wgpu::Adapter,
    surface_config: RefCell<wgpu::SurfaceConfiguration>,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl<'w> State<'w> {
    pub async fn new(target: impl Into<wgpu::SurfaceTarget<'w>>, width: u32, height: u32) -> Self {
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

    pub fn create_buffer(
        &self,
        label: Option<&str>,
        size: usize,
        usage: wgpu::BufferUsages,
    ) -> wgpu::Buffer {
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label,
            size: size as wgpu::BufferAddress,
            usage,
            mapped_at_creation: false,
        })
    }

    pub fn create_texture(
        &self,
        label: Option<&str>,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        usage: wgpu::TextureUsages,
    ) -> wgpu::Texture {
        let mut desc = wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage,
            view_formats: &[],
        };

        if let Some(label) = label {
            let label = label.to_owned() + " texture";
            desc.label = Some(label.as_str());
            self.device.create_texture(&desc)
        } else {
            self.device.create_texture(&desc)
        }
    }

    pub fn create_buffer_mat4(
        &self,
        label: Option<&str>,
        usage: wgpu::BufferUsages,
    ) -> wgpu::Buffer {
        self.create_buffer(label, size_of::<f32>() * 16, usage)
    }

    pub fn create_depth_texture(
        &self,
        label: Option<&str>,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
    ) -> wgpu::Texture {
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT;

        if let Some(label) = label {
            let label = label.to_owned() + " depth";
            self.create_texture(Some(label.as_str()), width, height, format, usage)
        } else {
            self.create_texture(label, width, height, format, usage)
        }
    }

    pub fn create_bind_group(
        &self,
        label: Option<&str>,
        layout_entries: &[wgpu::BindGroupLayoutEntry],
        entries: &[wgpu::BindGroupEntry],
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let layout = if let Some(label) = label {
            let label = label.to_owned() + " layout";

            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some(label.as_str()),
                    entries: layout_entries,
                })
        } else {
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: layout_entries,
                })
        };

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout: &layout,
            entries,
        });

        (layout, bind_group)
    }

    pub fn create_view_proj(
        &self,
        label: Option<&str>,
        binding: u32,
    ) -> (wgpu::Buffer, wgpu::BindGroupLayoutEntry) {
        let usage = wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST;

        let buffer = if let Some(label) = label {
            self.create_buffer_mat4(
                Some((label.to_owned() + " view projection matrix").as_str()),
                usage,
            )
        } else {
            self.create_buffer_mat4(None, usage)
        };

        (
            buffer,
            wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        )
    }

    pub fn get_target_state(&self, blend: wgpu::BlendState) -> wgpu::ColorTargetState {
        let format = self.surface.get_capabilities(&self.adapter).formats[0];
        let blend = Some(blend);

        wgpu::ColorTargetState {
            format,
            blend,
            write_mask: wgpu::ColorWrites::ALL,
        }
    }

    pub fn create_render_pipeline(
        &self,
        label: Option<&str>,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        vert_state: wgpu::VertexState,
        frag_state: wgpu::FragmentState,
        raster_state: wgpu::PrimitiveState,
        depth_stencil: Option<wgpu::DepthStencilState>,
    ) -> (wgpu::PipelineLayout, wgpu::RenderPipeline) {
        let pipeline_layout = if let Some(label) = label {
            let label = label.to_owned() + " pipeline layout";

            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some(label.as_str()),
                    bind_group_layouts: bind_group_layouts,
                    push_constant_ranges: &[],
                })
        } else {
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: bind_group_layouts,
                    push_constant_ranges: &[],
                })
        };

        let mut pipeline_desc = wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: vert_state,
            fragment: Some(frag_state),
            primitive: raster_state,
            depth_stencil: depth_stencil,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        };

        let pipeline = if let Some(label) = label {
            let label = label.to_owned() + " pipeline";

            pipeline_desc.label = Some(label.as_str());
            self.device.create_render_pipeline(&pipeline_desc)
        } else {
            self.device.create_render_pipeline(&pipeline_desc)
        };

        (pipeline_layout, pipeline)
    }

    pub fn get_frame(&self) -> wgpu::SurfaceTexture {
        self.surface
            .get_current_texture()
            .expect("failed to acquire next swap chain texture")
    }

    pub fn create_command_encoder(&self, label: Option<&str>) -> wgpu::CommandEncoder {
        if let Some(label) = label {
            let label = label.to_owned() + " command encoder";
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some(label.as_str()),
                })
        } else {
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label })
        }
    }

    pub fn submit(&self, commands: wgpu::CommandBuffer) {
        self.queue.submit(Some(commands));
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

pub fn get_vert_state<'s, 'b: 's>(
    shader: &'s wgpu::ShaderModule,
    buffers: &'b [wgpu::VertexBufferLayout<'b>],
) -> wgpu::VertexState<'s> {
    wgpu::VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers,
        compilation_options: Default::default(),
    }
}

pub fn get_frag_state<'s, 't: 's>(
    shader: &'s wgpu::ShaderModule,
    targets: &'t [Option<wgpu::ColorTargetState>],
) -> wgpu::FragmentState<'s> {
    wgpu::FragmentState::<'s> {
        module: shader,
        entry_point: "fs_main",
        compilation_options: wgpu::PipelineCompilationOptions::<'s>::default(),
        targets,
    }
}

pub fn get_raster_state(cull: bool) -> wgpu::PrimitiveState {
    wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: if cull { Some(wgpu::Face::Back) } else { None },
        unclipped_depth: false,
        polygon_mode: wgpu::PolygonMode::Fill,
        conservative: false,
    }
}

pub fn begin_render_pass<'p>(
    encoder: &'p mut wgpu::CommandEncoder,
    color_attachments: &[Option<wgpu::RenderPassColorAttachment<'p>>],
    depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'p>>,
) -> wgpu::RenderPass<'p> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments,
        depth_stencil_attachment,
        timestamp_writes: None,
        occlusion_query_set: None,
    })
}
