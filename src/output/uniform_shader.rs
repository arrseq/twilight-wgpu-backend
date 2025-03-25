use std::simd::f32x2;
use bytemuck::{Pod, Zeroable};
use wgpu::{include_wgsl, vertex_attr_array, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer, BufferAddress, BufferBindingType, BufferDescriptor, BufferSize, BufferUsages, ColorTargetState, Device, FragmentState, PipelineLayoutDescriptor, RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderStages, TextureFormat, VertexBufferLayout, VertexState, VertexStepMode};
use crate::output::{RenderObject};

#[derive(Debug)]
pub struct UniformShader {
    pub render_object: RenderObject,
    color: Buffer
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct Vertex {
    position: [f32; 2]
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct Color {
    color: [f32; 4]
}

impl Color {
    pub const LAYOUT: BindGroupLayoutEntry = BindGroupLayoutEntry {
        binding: 0,
        visibility: ShaderStages::FRAGMENT,
        ty: BindingType::Buffer {
            min_binding_size: None,
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false
        },
        count: None,
    };
    
    pub fn create(device: &Device) -> Buffer {
        device.create_buffer(&BufferDescriptor {
            label: None,
            size: size_of::<Self>() as BufferAddress,
            usage: BufferUsages::UNIFORM,
            mapped_at_creation: false
        })
    }
}

impl Vertex {
    pub const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        attributes: &vertex_attr_array![0 => Float32x2],
        step_mode: VertexStepMode::Vertex,
        array_stride: size_of::<Vertex>() as BufferAddress
    };
}

impl UniformShader {
    pub(crate) fn new(device: &Device, format: TextureFormat) -> Self {
        let shader = device.create_shader_module(include_wgsl!("./uniform.wgsl"));
        
        let color = Color::create(&device);
        
        let bind_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[Color::LAYOUT]
        });
        
        let bind = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: color.as_entire_binding()
            }],
            layout: &bind_layout
        });
        
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_layout],
            push_constant_ranges: &[]
        });
        
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Vertex::LAYOUT]
            },
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[
                    Some(ColorTargetState {
                        format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })
                ]
            }),
            multiview: None,
            cache: None,
        });
        
        Self {
            render_object: RenderObject {
                pipeline,
                bind_group: Some(bind)
            },
            color            
        }
    }
}