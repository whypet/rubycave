use std::{borrow::Borrow, marker::PhantomData};

use crate::resource::ResourceManager;

use super::{Renderer, State};

pub struct TriangleRenderer<'state, StateRef: Borrow<State<'state>>> {
    _phantom: PhantomData<&'state ()>,
    state_ref: StateRef,
    render_pipeline: wgpu::RenderPipeline,
}

impl<'state, StateRef: Borrow<State<'state>>> Renderer<'state, StateRef>
    for TriangleRenderer<'state, StateRef>
{
    fn new(state_ref: StateRef, resource_man: &'state ResourceManager) -> Self {
        let mut res = resource_man.get(crate::resource::DIR_SHADER.to_owned() + "/triangle.wgsl");
        let source = res.read_to_str().expect("failed to read triangle shader");

        let state = state_ref.borrow();
        let shader = state
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(source.as_str().into()),
            });

        let pipeline_layout =
            state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[],
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
                    primitive: wgpu::PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    multiview: None,
                });

        Self {
            _phantom: PhantomData,
            state_ref,
            render_pipeline,
        }
    }

    fn render(&self) {
        let state = self.state_ref.borrow();

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
            rpass.draw(0..3, 0..1);
        }

        state.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
