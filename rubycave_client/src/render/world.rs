use std::{cell::RefCell, rc::Rc};

use rubycave::glam::Mat4;

use crate::{
    config::Config,
    resource::{self, ResourceManager},
};

use super::{
    view::{self, Camera},
    Renderer, SizedSurface, State,
};

const LABEL: &str = "Chunk renderer";
const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;

pub struct ChunkRenderer<'a> {
    state: Rc<State<'a>>,
    config: Rc<Config>,

    render_pipeline: wgpu::RenderPipeline,
    depth_view: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
    vp_buffer: wgpu::Buffer,

    vp: Option<Mat4>,
    camera: Rc<RefCell<Camera>>,

    fov: f32,
}

impl<'a> ChunkRenderer<'a> {
    pub fn new(
        state: Rc<State<'a>>,
        config: Rc<Config>,
        resource_man: Rc<ResourceManager>,
        camera: Rc<RefCell<Camera>>,
    ) -> Result<Self, resource::Error> {
        let device: &wgpu::Device = &state.device;

        let mut res = resource_man.get(crate::SHADER_DIR.to_owned() + "/triangle.wgsl");
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

        let (width, height) = state.surface_config.get_size();
        let depth_view = Self::create_depth_view(&state, width, height);

        let tgt_state = state.get_target_state(wgpu::BlendState::REPLACE);

        let (_, render_pipeline) = state.create_render_pipeline(
            Some(LABEL),
            &[&bind_group_layout],
            super::get_vert_state(&shader, &[]),
            super::get_frag_state(&shader, &[Some(tgt_state)]),
            super::get_raster_state(false),
            Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
        );

        Ok(Self {
            state,
            config,

            render_pipeline,
            depth_view,
            bind_group,
            vp_buffer,

            vp: None,
            camera,

            fov: 0.0,
        })
    }

    fn create_depth_view(state: &State, width: u32, height: u32) -> wgpu::TextureView {
        state
            .create_depth_texture(Some(LABEL), width, height, DEPTH_FORMAT)
            .create_view(&wgpu::TextureViewDescriptor::default())
    }

    /*
    fn create_terrain_texture(state: &State, resource_man: Rc<ResourceManager>) {
        let mut res = resource_man.get(crate::resource::DIR_TEXTURE.to_owned() + "/terrain.png");
        let png = PngDecoder::new(BufReader::new(
            res.open().expect("failed to open terrain atlas texture"),
        ))
        .expect("failed to decode terrain atlas texture as png");
    }
    */
}

impl Renderer for ChunkRenderer<'_> {
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
                Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
            );

            pass.set_pipeline(&self.render_pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.draw(0..3, 0..1);
        }

        encoder.finish()
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.depth_view = Self::create_depth_view(&self.state, width, height)
    }
}
