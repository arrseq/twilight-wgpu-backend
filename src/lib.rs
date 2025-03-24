#![feature(portable_simd)]
#![feature(mpmc_channel)]
#![feature(type_alias_impl_trait)]
#![feature(fn_traits)]

pub mod output;
pub mod shape;

use std::{iter, mem};
use std::num::NonZeroU32;
use std::simd::usizex2;
use std::sync::Arc;
use bytemuck::{cast_slice, Pod, Zeroable};
use wgpu::{include_wgsl, vertex_attr_array, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer, BufferAddress, BufferBindingType, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Device, LoadOp, MapMode, Operations, PipelineLayoutDescriptor, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderStages, StoreOp, Surface, SurfaceConfiguration, TextureFormat, TextureViewDescriptor, VertexAttribute, VertexBufferLayout, VertexStepMode};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::output::Output;
