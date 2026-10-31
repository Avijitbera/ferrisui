use crate::batch::RenderBatch;
use crate::clip::ClipStack;
use crate::context::GpuContext;
use crate::pipline::PipelineManager;
use crate::primitives::{RectInstance, WindowUniforms};
use bytemuck::cast_slice;

use ferrisui_core::color::Color;
use ferrisui_core::geometry::{Rect, Size};
use wgpu::util::DeviceExt;
use wgpu::*;

pub struct Renderer {
    pub context: GpuContext,
    pub pipelines: PipelineManager,
    pub clip_stack: ClipStack,
    pub batch: RenderBatch,
    uniform_buffer: Buffer,
    uniform_bind_group: BindGroup,
}

impl Renderer {
    pub fn new(context: GpuContext) -> Self {
        let pipelines = PipelineManager::new(&context.device, context.surface_config.format);
        let initial_uniforms = WindowUniforms::new(Size::new(
            context.surface_config.width as f32,
            context.surface_config.height as f32,
        ));

        let uniform_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: cast_slice(&[initial_uniforms]),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            });

        let uniform_bind_group = context.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Uniform Bind Grouo"),
            layout: &pipelines.uniform_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            context,
            pipelines,
            clip_stack: ClipStack::new(),
            batch: RenderBatch::new(),
            uniform_buffer,
            uniform_bind_group,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.context.resize(width, height);
        let uniforms = WindowUniforms::new(Size::new(width as f32, height as f32));
        self.context
            .queue
            .write_buffer(&self.uniform_buffer, 0, cast_slice(&[uniforms]));
    }

    pub fn resize_logical(&mut self, width: f32, height: f32) {
        let uniforms = WindowUniforms::new(Size::new(width, height));
        self.context
            .queue
            .write_buffer(&self.uniform_buffer, 0, cast_slice(&[uniforms]));
    }

    pub fn draw_rect(
        &mut self,
        rect: Rect,
        color: Color,
        radius: f32,
        border_width: f32,
        border_color: Color,
    ) {
        self.batch.push_rect(RectInstance::new(
            rect,
            color,
            radius,
            border_width,
            border_color,
        ));
    }

    pub fn render_frame(&mut self, clear_color: Color) -> Result<(), SurfaceError> {
        let output = self.context.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .context
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let rgba = clear_color.to_rgba_f32();

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("UI Main Pass"),
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(wgpu::Color {
                        r: rgba[0] as f64,
                        g: rgba[1] as f64,
                        b: rgba[2] as f64,
                        a: rgba[3] as f64,
                    }),
                    store: StoreOp::Store,
                },
            })],
        });

        if !self.batch.rects.is_empty() {
            let instance_buffer =
                self.context
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Rect Instance Buffer"),
                        contents: cast_slice(&self.batch.rects),
                        usage: BufferUsages::VERTEX,
                    });
            render_pass.set_pipeline(&self.pipelines.rect_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, instance_buffer.slice(..));
            render_pass.draw(0..6, 0..self.batch.rects.len() as u32);
        }

        drop(render_pass);

        self.context.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        self.batch.clear();
        Ok(())
    }
}
