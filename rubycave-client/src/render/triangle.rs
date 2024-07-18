use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use rubycave::glam::Mat4;

use crate::{config::Config, resource::ResourceManager};

use super::{
    view::{self, Camera},
    Renderer, SizedSurface, State,
};

pub struct TriangleRenderer<'a> {
    state: Rc<State<'a>>,
    config: Rc<Config>,

    render_pipeline: wgpu::RenderPipeline,
    view_proj_buffer: wgpu::Buffer,
    view_proj_bind_group: wgpu::BindGroup,

    view_proj: RefCell<Option<Mat4>>,
    camera: Rc<RefCell<Camera>>,

    fov: Cell<f32>,
}

impl<'a> TriangleRenderer<'a> {
    pub fn new(
        state: Rc<State<'a>>,
        config: Rc<Config>,
        resource_man: Rc<ResourceManager>,
        camera: Rc<RefCell<Camera>>,
    ) -> Self {
        let mut res = resource_man.get(crate::resource::DIR_SHADER.to_owned() + "/triangle.wgsl");
        let source = res.read_to_str().expect("failed to read triangle shader");

        let surface: &wgpu::Surface = &state.surface;
        let adapter: &wgpu::Adapter = &state.adapter;
        let device: &wgpu::Device = &state.device;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(source.as_str().into()),
        });

        let view_proj_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view_proj_buffer"),
            size: (size_of::<f32>() * 16) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let view_proj_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("view_proj_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let view_proj_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view_proj_bind_group"),
            layout: &view_proj_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: view_proj_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&view_proj_bind_group_layout],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: swapchain_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self {
            state,
            config,

            view_proj_bind_group,
            view_proj_buffer,
            render_pipeline,

            view_proj: RefCell::new(None),
            camera,

            fov: Cell::new(0.0),
        }
    }
}

impl Renderer for TriangleRenderer<'_> {
    fn render(&self) {
        let surface: &wgpu::Surface = &self.state.surface;
        let device: &wgpu::Device = &self.state.device;
        let queue: &wgpu::Queue = &self.state.queue;

        let camera = self.camera.borrow();
        let config_fov = self.config.get_fov();

        if camera.is_updated() || self.fov.get() != config_fov {
            let (width, height) = self.state.surface_config.get_size();

            self.fov.set(config_fov);
            *self.view_proj.borrow_mut() = Some(
                view::perspective_rh(&self.config, width as f32, height as f32) * camera.view(),
            );
        }

        queue.write_buffer(
            &self.view_proj_buffer,
            0,
            bytemuck::cast_slice(AsRef::<[f32; 16]>::as_ref(
                self.view_proj.borrow().as_ref().unwrap(),
            )),
        );

        let frame = surface
            .get_current_texture()
            .expect("failed to acquire next swap chain texture");

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, &self.view_proj_bind_group, &[]);
            rpass.draw(0..3, 0..1);
        }

        queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
