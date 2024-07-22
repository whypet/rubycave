use std::{cell::RefCell, path::Path, rc::Rc};

use rubycave::glam::Mat4;

use crate::{
    config::Config,
    resource::{self, ResourceManager},
};

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
    ) -> Result<Self, resource::Error> {
        let device: &wgpu::Device = &state.device;

        let mut res =
            resource_man.get_from_path(&Path::new(crate::SHADER_DIR).join("triangle.wgsl"))?;
        let source = res.read_to_str()?;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(LABEL),
            source: wgpu::ShaderSource::Wgsl(source.as_str().into()),
        });

        let (vp_buffer, vp_entry) = state.create_view_proj(Some(LABEL), 0);

        let (bind_group_layout, bind_group) = state.create_bind_group(
            Some(LABEL),
            &[vp_entry],
            &[wgpu::BindGroupEntry {
                binding: vp_entry.binding,
                resource: vp_buffer.as_entire_binding(),
            }],
        );

        let tgt_state = state.get_target_state(wgpu::BlendState::REPLACE);

        let (_, render_pipeline) = state.create_render_pipeline(
            Some(LABEL),
            &[&bind_group_layout],
            super::get_vert_state(&shader, &[]),
            super::get_frag_state(&shader, &[Some(tgt_state)]),
            super::get_raster_state(false),
            None,
        );

        Ok(Self {
            state,
            config,

            render_pipeline,
            bind_group,
            vp_buffer,

            vp: None,
            camera,

            fov: 0.0,
        })
    }
}

impl Renderer for TriangleRenderer<'_> {
    fn update(&mut self) {
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
    }

    fn render<'p, 'a: 'p>(&'a mut self, frame_view: &wgpu::TextureView) -> wgpu::CommandBuffer {
        let mut encoder = self.state.create_command_encoder(Some(LABEL));

        {
            let mut pass = super::begin_render_pass(
                &mut encoder,
                &[Some(wgpu::RenderPassColorAttachment {
                    view: frame_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                None,
            );

            pass.set_pipeline(&self.render_pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.draw(0..3, 0..1);
        }

        encoder.finish()
    }

    fn resize(&mut self, _: u32, _: u32) {}
}
