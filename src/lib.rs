#![feature(portable_simd)]
#![feature(mpmc_channel)]
#![feature(type_alias_impl_trait)]
#![feature(fn_traits)]

mod output;
pub mod shape;

use std::{iter, mem};
use std::num::NonZeroU32;
use std::simd::usizex2;
use std::sync::Arc;
use bytemuck::{cast_slice, Pod, Zeroable};
use wgpu::{include_wgsl, vertex_attr_array, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer, BufferAddress, BufferBindingType, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Device, LoadOp, MapMode, Operations, PipelineLayoutDescriptor, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderStages, StoreOp, Surface, SurfaceConfiguration, TextureFormat, TextureViewDescriptor, VertexAttribute, VertexBufferLayout, VertexStepMode};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::output::Output;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniform {
    color_it: u32
}

#[derive(Debug)]
pub struct Backend {
    pub size: usizex2,
    output: Output
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3]
}

impl Backend {
    pub fn new(device: Device, size: usizex2, format: TextureFormat) -> Self {

        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("uniform"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            size: size_of::<Uniform>() as BufferAddress,
            mapped_at_creation: false
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("BG layout"),
            entries: &[
                BindGroupLayoutEntry {
                    visibility: ShaderStages::all(),
                    binding: 0,
                    count: None,
                    ty: BindingType::Buffer {
                        has_dynamic_offset: false,
                        ty: BufferBindingType::Uniform,
                        min_binding_size: None
                    }
                },
            ]
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding()
                }
            ],
            label: Some("BG")
        });
        
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // 1.
                buffers: &[
                    VertexBufferLayout {
                        array_stride: size_of::<Vertex>() as BufferAddress,
                        step_mode: VertexStepMode::Vertex,
                        attributes: &Self::ATTRIBUTES
                    }
                ], // 2.
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
            cache: None, // 6.
        });
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("buffer"),
            usage: BufferUsages::VERTEX,
            contents: cast_slice(&[
                Vertex { position: [-1.0, -1.0, 0.0], color: [1.0, 0.0, 0.0] },
                Vertex { position: [1.0, -1.0, 0.0], color: [0.0, 1.0, 0.0] },
                Vertex { position: [0.0, 1.0, 0.0], color: [0.0, 0.0, 1.0] },

                Vertex { position: [0.0, 1.0, 0.0], color: [0.0, 0.0, 1.0] },
                Vertex { position: [1.0, -1.0, 0.0], color: [0.0, 1.0, 1.0] },
                Vertex { position: [1.0, 1.0, 0.0], color: [0.0, 0.0, 0.0] },
            ])
        });
        
        Self {
            pipeline,
            vertex_buffer,
            uniform_buffer: Arc::new(uniform_buffer),
            bind_group,
            size
        }
    }
    
    pub fn render(&mut self, pass: &mut RenderPass) {
        // let cloned = self.uniform_buffer.clone();
        // self.uniform_buffer.slice(..).map_async(MapMode::Write, move |b| {
        //     let Ok(_) = b else { return };
        //     for byte in cloned.slice(..).get_mapped_range_mut().iter_mut() {
        //         *byte = if color_it { 1 } else { 0 };
        //     }
        //     cloned.unmap();
        // });
        
        let staging = self.device.create_buffer_init(&BufferInitDescriptor {
            usage: BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            label: Some("staging"),
            contents: &(color_it as u32).to_le_bytes()
        });
        
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&TextureViewDescriptor {
            ..Default::default()
        });

        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Pass")
        });
        
        encoder.copy_buffer_to_buffer(&staging, 0, &self.uniform_buffer, 0, size_of::<Uniform>() as BufferAddress);

        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..6, 0..1);

        drop(pass);

        self.queue.submit(iter::once(encoder.finish()));
        output.present();
    }
}
