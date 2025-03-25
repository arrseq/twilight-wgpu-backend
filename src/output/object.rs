use std::collections::LinkedList;
use std::simd::f32x4;
use wgpu::Buffer;

#[derive(Debug)]
pub struct ObjectClass {
    pub shape_id: usize,
    // pub objects: Box<[Object]>,
    // pub vertexes: Buffer,
    pub shader: ObjectShader
}

#[derive(Debug)]
pub struct Object {
    pub indexes: Buffer,
    pub vertex_count: u32,
}

#[derive(Debug, PartialEq)]
pub enum ObjectShader {
    /// A uniformly colored object
    Uniform { 
        color: f32x4,
        buffer: Buffer
    },
    /// Colored based on the color of vertexes
    Vertex {
        buffer: Buffer
    },
    // TODO
    BlurBehind {
        buffer: Buffer,
        radius: f32
    },
    // TODO
    Texture
}