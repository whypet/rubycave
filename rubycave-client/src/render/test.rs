use std::{cell::RefCell, rc::Rc};

use rubycave::glam::Mat4;
use wgpu::BlendState;

use crate::{config::Config, resource::ResourceManager};

use super::{
    view::{self, Camera},
    Renderer, SizedSurface, State,
};

const LABEL: &str = "Triangle renderer";

pub struct TriangleRenderer<'a> {
    state: Rc<State<'a>>,
    config: Rc<Config>,

    render_pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    vp_buffer: wgpu::Buffer,

    vp: Option<Mat4>,
    camera: Rc<RefCell<Camera>>,

    fov: f32,
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
            label: Some(LABEL),
            source: wgpu::ShaderSource::Wgsl(source.as_str().into()),
        });

        let (vp_buffer, vp_entry) = super::create_view_proj(device, Some(LABEL), 0);

        let (bind_group_layout, bind_group) = super::create_bind_group(
            device,
            Some(LABEL),
            &[vp_entry],
            &[wgpu::BindGroupEntry {
                binding: vp_entry.binding,
                resource: vp_buffer.as_entire_binding(),
            }],
        );

        let swap_format = super::get_swap_format(surface, adapter);
        let tgt_state = super::get_target_state(swap_format, BlendState::REPLACE);

        let (_, render_pipeline) = super::create_render_pipeline(
            device,
            Some(LABEL),
            &[&bind_group_layout],
            super::get_vert_state(&shader, &[]),
            super::get_frag_state(&shader, &[Some(tgt_state)]),
            super::get_raster_state(false),
            None,
        );

        Self {
            state,
            config,

            render_pipeline,
            bind_group,
            vp_buffer,

            vp: None,
            camera,

            fov: 0.0,
        }
    }
}

impl Renderer for TriangleRenderer<'_> {
    fn render(&mut self) {
        let surface: &wgpu::Surface = &self.state.surface;
        let device: &wgpu::Device = &self.state.device;
        let queue: &wgpu::Queue = &self.state.queue;

        let camera = self.camera.borrow();
        let config_fov = self.config.fov;

        if self.vp.is_none() || camera.is_updated() || self.fov != config_fov {
            let (width, height) = self.state.surface_config.get_size();

            self.fov = config_fov;
            self.vp = Some(view::perspective_rh(self.fov, width, height) * camera.view());
        }

        queue.write_buffer(
            &self.vp_buffer,
            0,
            bytemuck::cast_slice(AsRef::<[f32; 16]>::as_ref(self.vp.as_ref().unwrap())),
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
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.draw(0..3, 0..1);
        }

        queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
