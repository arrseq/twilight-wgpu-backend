use std::collections::LinkedList;
use std::simd::f32x4;
use wgpu::Buffer;

#[derive(Debug)]
pub struct ShapeObjects {
    pub shape_id: usize,
    pub objects: Vec<Object>,
    pub vertexes: Buffer
}

#[derive(Debug)]
pub struct Object {
    pub indexes: Buffer,
    pub vertex_count: u32,
    /// Index buffer
    pub kind: ObjectKind
}

#[derive(Debug)]
pub enum ObjectKind {
    /// A uniformly colored object
    Uniform(f32x4),
    /// Colored based on the color of vertexes
    Vertex,
    // TODO
    BlurBehind,
    // TODO
    Texture
}