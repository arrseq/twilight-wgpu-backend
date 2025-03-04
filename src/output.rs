pub mod uniform_shader;
mod object;

use crate::output::object::ShapeObjects;
use crate::output::uniform_shader::UniformShader;
use std::fmt::Debug;
use wgpu::{Device, IndexFormat, RenderPass, RenderPipeline, TextureFormat};

pub trait Shader: Debug {
    fn new(device: &Device, format: TextureFormat) -> Self;
    fn render(&mut self, pass: RenderPass);
    fn pipeline(&self) -> &RenderPipeline;
}

#[derive(Debug)]
pub struct Output {
    shape_objects: Vec<ShapeObjects>,
    uniform_shader: UniformShader
}

impl Output {
    fn new(device: Device, format: TextureFormat) -> Self {
        Self {
            shape_objects: Vec::new(),
            uniform_shader: Shader::new(&device, format)
        }
    }
    
    fn render(&self, pass: &mut RenderPass) {
        pass.set_pipeline(self.uniform_shader.pipeline());
        
        for shape in &self.shape_objects {
            pass.set_vertex_buffer(0, shape.vertexes.slice(..));
            for object in &shape.objects {
                pass.set_index_buffer(object.indexes.slice(..), IndexFormat::Uint32);
                pass.draw(0..object.indexes.size() as u32, 0..1);
            }
        }
        
    }
}