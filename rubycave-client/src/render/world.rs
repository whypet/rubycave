use std::{cell::RefCell, rc::Rc};

use rubycave::glam::Mat4;

use crate::config::Config;

use super::{
    view::{self, Camera},
    Renderer, SizedSurface, State,
};

pub struct WorldRenderer<'a> {
    state: Rc<State<'a>>,
    config: Rc<Config>,

    bind_group: wgpu::BindGroup,
    vp_buffer: wgpu::Buffer,

    view_proj: Option<Mat4>,
    camera: Rc<RefCell<Camera>>,

    fov: f32,
}

impl<'a> WorldRenderer<'a> {
    pub fn new(state: Rc<State<'a>>, config: Rc<Config>, camera: Rc<RefCell<Camera>>) -> Self {
        let device: &wgpu::Device = &state.device;

        let (vp_buffer, vp_entry) = super::create_view_proj(device, Some("Triangle renderer"), 0);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("view_proj_bind_group_layout"),
            entries: &[vp_entry],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("view_proj_bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: vp_entry.binding,
                resource: vp_buffer.as_entire_binding(),
            }],
        });

        Self {
            state,
            config,

            bind_group,
            vp_buffer,

            view_proj: None,
            camera,

            fov: 0.0,
        }
    }
}

impl Renderer for WorldRenderer<'_> {
    fn render(&mut self) {
        let surface: &wgpu::Surface = &self.state.surface;
        let device: &wgpu::Device = &self.state.device;
        let queue: &wgpu::Queue = &self.state.queue;

        let camera = self.camera.borrow();
        let config_fov = self.config.fov;

        if self.view_proj.is_none() || camera.is_updated() || self.fov != config_fov {
            let (width, height) = self.state.surface_config.get_size();

            self.fov = config_fov;
            self.view_proj = Some(view::perspective_rh(self.fov, width, height) * camera.view());
        }

        queue.write_buffer(
            &self.vp_buffer,
            0,
            bytemuck::cast_slice(AsRef::<[f32; 16]>::as_ref(self.view_proj.as_ref().unwrap())),
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
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.draw(0..3, 0..1);
        }

        queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
