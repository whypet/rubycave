use std::{cell::RefCell, mem, path::Path, rc::Rc};

use bytemuck::{Pod, Zeroable};
use rubycave::glam::{Mat4, Vec2, Vec3};

use crate::{config::Config, render, resource::ResourceManager, SHADER_DIR, TEXTURE_DIR};

use super::{
    view::{self, Camera},
    Renderer, SizedSurface, State, Vertex,
};

const LABEL: &str = "Chunk renderer";
const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth24Plus;
const TEST_VERTICES: &[ChunkVertex] = &[
    ChunkVertex {
        position: Vec3::new(0.0, 0.5, -3.0),
        tex_coords: Vec2::new(0.0, 0.0),
    },
    ChunkVertex {
        position: Vec3::new(-0.5, -0.5, -3.0),
        tex_coords: Vec2::new(0.25, 0.0),
    },
    ChunkVertex {
        position: Vec3::new(0.5, -0.5, -3.0),
        tex_coords: Vec2::new(0.0, 0.25),
    },
];

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ChunkVertex {
    position: Vec3,
    tex_coords: Vec2,
}

pub struct ChunkRenderer<'a> {
    state: Rc<State<'a>>,
    config: Rc<Config>,

    render_pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    depth_view: wgpu::TextureView,

    vp: (wgpu::Buffer, Option<Mat4>),
    vertex_buffer: wgpu::Buffer,

    camera: Rc<RefCell<Camera>>,
    fov: f32,
}

impl ChunkVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];
}

impl<'a> Vertex<'a> for ChunkVertex {
    fn desc() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

impl<'a> ChunkRenderer<'a> {
    pub fn new(
        state: Rc<State<'a>>,
        config: Rc<Config>,
        resource_man: Rc<ResourceManager>,
        camera: Rc<RefCell<Camera>>,
    ) -> Result<Self, render::Error> {
        let device = &state.device;
        let queue = &state.queue;

        let mut res = resource_man.get_from_path(&Path::new(SHADER_DIR).join("chunk.wgsl"))?;
        let source = res.read_to_str()?;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(LABEL),
            source: wgpu::ShaderSource::Wgsl(source.as_str().into()),
        });

        let (vp_buffer, vp_entry) = state.create_view_proj(Some(LABEL), 0);
        let vp = (vp_buffer, None);

        let (sampler, sampler_entry) = state.create_sampler(
            Some(LABEL),
            1,
            &wgpu::SamplerDescriptor {
                label: None,
                address_mode_u: wgpu::AddressMode::Repeat,
                address_mode_v: wgpu::AddressMode::Repeat,
                address_mode_w: wgpu::AddressMode::Repeat,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..wgpu::SamplerDescriptor::default()
            },
            false,
        );

        let (terrain_atlas, terrain_atlas_entry) =
            Self::create_terrain_texture(2, &state, resource_man)?;
        let terrain_atlas_view = terrain_atlas.create_view(&wgpu::TextureViewDescriptor::default());

        let (bind_group_layout, bind_group) = state.create_bind_group(
            Some(LABEL),
            &[vp_entry, sampler_entry, terrain_atlas_entry],
            &[
                wgpu::BindGroupEntry {
                    binding: vp_entry.binding,
                    resource: vp.0.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: sampler_entry.binding,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: terrain_atlas_entry.binding,
                    resource: wgpu::BindingResource::TextureView(&terrain_atlas_view),
                },
            ],
        );

        let (_, render_pipeline) = state.create_render_pipeline(
            Some(LABEL),
            &[&bind_group_layout],
            super::get_vert_state(&shader, &[ChunkVertex::desc()]),
            super::get_frag_state(
                &shader,
                &[Some(state.get_target_state(wgpu::BlendState::REPLACE))],
            ),
            super::get_raster_state(false),
            Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
        );

        let (width, height) = state.surface_config.get_size();
        let depth_view = Self::create_depth_view(&state, width, height);

        let vertex_buffer = state.create_buffer(
            Some((LABEL.to_owned() + " vertex").as_str()),
            mem::size_of::<[ChunkVertex; 3]>(),
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        );

        queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(TEST_VERTICES));

        Ok(Self {
            state,
            config,

            render_pipeline,
            bind_group,
            depth_view,

            vp,
            vertex_buffer,

            camera,
            fov: 0.0,
        })
    }

    fn create_depth_view(state: &State, width: u32, height: u32) -> wgpu::TextureView {
        state
            .create_depth_texture(Some(LABEL), width, height, DEPTH_FORMAT)
            .create_view(&wgpu::TextureViewDescriptor::default())
    }

    fn create_terrain_texture(
        binding: u32,
        state: &State,
        resource_man: Rc<ResourceManager>,
    ) -> Result<(wgpu::Texture, wgpu::BindGroupLayoutEntry), render::Error> {
        let mut res = resource_man.get_from_path(&Path::new(TEXTURE_DIR).join("terrain.png"))?;

        state.load_texture(
            Some((LABEL.to_owned() + " terrain").as_str()),
            binding,
            res.open()?,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            false,
        )
    }
}

impl Renderer for ChunkRenderer<'_> {
    fn update(&mut self) {
        let queue: &wgpu::Queue = &self.state.queue;

        let camera = self.camera.borrow();
        let config_fov = self.config.fov;

        if self.vp.1.is_none() || camera.is_updated() || self.fov != config_fov {
            let (width, height) = self.state.surface_config.get_size();

            self.fov = config_fov;
            self.vp.1 = Some(view::perspective_rh(self.fov, width, height) * camera.view());
        }

        queue.write_buffer(
            &self.vp.0,
            0,
            bytemuck::cast_slice(AsRef::<[f32; 16]>::as_ref(self.vp.1.as_ref().unwrap())),
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
            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            pass.draw(0..3, 0..1);
        }

        encoder.finish()
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.depth_view = Self::create_depth_view(&self.state, width, height)
    }
}
