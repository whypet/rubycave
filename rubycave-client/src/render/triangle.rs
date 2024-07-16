use std::{borrow::Borrow, marker::PhantomData};

use crate::resource::ResourceManager;

use super::{
    view::{self, Camera},
    Renderer, State,
};

pub struct TriangleRenderer<'a, StateRef: Borrow<State<'a>>> {
    _phantom: PhantomData<&'a ()>,
    state_ref: StateRef,

    render_pipeline: wgpu::RenderPipeline,
    view_proj_buffer: wgpu::Buffer,
    view_proj_bind_group: wgpu::BindGroup,

    proj: [f32; 16],
    camera: &'a Camera,
}

impl<'a, StateRef: Borrow<State<'a>>> TriangleRenderer<'a, StateRef> {
    pub fn new(state_ref: StateRef, resource_man: &'a ResourceManager, camera: &'a Camera) -> Self {
        let mut res = resource_man.get(crate::resource::DIR_SHADER.to_owned() + "/triangle.wgsl");
        let source = res.read_to_str().expect("failed to read triangle shader");

        let state = state_ref.borrow();

        let shader = state
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(source.as_str().into()),
            });

        let view_proj_buffer = state.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view_proj_buffer"),
            size: (size_of::<f32>() * 16) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let view_proj_bind_group_layout =
            state
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let view_proj_bind_group = state.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view_proj_bind_group"),
            layout: &view_proj_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: view_proj_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout =
            state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&view_proj_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let swapchain_capabilities = state.surface.get_capabilities(&state.adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let render_pipeline =
            state
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                        targets: &[Some(swapchain_format.into())],
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
            _phantom: PhantomData,
            state_ref,

            view_proj_bind_group,
            view_proj_buffer,
            render_pipeline,

            proj: view::ortho_proj(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0),
            camera,
        }
    }
}

impl<'a, StateRef: Borrow<State<'a>>> Renderer<'a, StateRef> for TriangleRenderer<'a, StateRef> {
    fn render(&self) {
        let state = self.state_ref.borrow();

        state.queue.write_buffer(
            &self.view_proj_buffer,
            0,
            bytemuck::cast_slice(&[self.proj]),
        );

        let frame = state
            .surface
            .get_current_texture()
            .expect("failed to acquire next swap chain texture");

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
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

        state.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
