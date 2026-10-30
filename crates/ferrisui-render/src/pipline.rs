use crate::primitives::RectInstance;
use std::mem;
use wgpu::*;

pub struct PipelineManager {
    pub rect_pipeline: RenderPipeline,
    pub uniform_bind_group_layout: BindGroupLayout,
}

impl PipelineManager {
    pub fn new(device: &Device, format: TextureFormat) -> Self {
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Rect Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Rect Shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/rect.wgsl").into()),
        });

        let rect_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Rect Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[VertexBufferLayout {
                    array_stride: mem::size_of::<RectInstance>() as BufferAddress,
                    step_mode: VertexStepMode::Instance,
                    attributes: &[
                        VertexAttribute {
                            format: VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 0,
                        },
                        VertexAttribute {
                            format: VertexFormat::Float32x2,
                            offset: 8,
                            shader_location: 1,
                        },
                        VertexAttribute {
                            format: VertexFormat::Float32x4,
                            offset: 16,
                            shader_location: 2,
                        },
                        VertexAttribute {
                            format: VertexFormat::Float32x4,
                            offset: 32,
                            shader_location: 3,
                        },
                        VertexAttribute {
                            format: VertexFormat::Float32,
                            offset: 48,
                            shader_location: 4,
                        },
                        VertexAttribute {
                            format: VertexFormat::Float32,
                            offset: 52,
                            shader_location: 5,
                        },
                    ],
                }],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            rect_pipeline,
            uniform_bind_group_layout,
        }
    }
}
