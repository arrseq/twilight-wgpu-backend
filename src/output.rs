pub mod uniform_shader;
mod object;

use crate::output::object::ShapeObjects;
use crate::output::uniform_shader::UniformShader;
use std::fmt::Debug;
use wgpu::{BindGroup, Device, IndexFormat, RenderPass, RenderPipeline, TextureFormat};
use wgpu::util::RenderEncoder;

#[derive(Debug)]
struct RenderObject {
    pipeline: RenderPipeline,
    bind_group: Option<BindGroup>
}

pub trait Shader: Debug {
    fn new(device: &Device, format: TextureFormat) -> Self;
    fn render_object(&self) -> &RenderObject;
}

#[derive(Debug)]
pub struct Output {
    shape_objects: Vec<ShapeObjects>,
    uniform_shader: UniformShader
}

impl Output {
    pub fn new(device: &Device, format: TextureFormat) -> Self {
        Self {
            shape_objects: Vec::new(),
            uniform_shader: Shader::new(device, format)
        }
    }
    
    pub fn render(&self, pass: &mut RenderPass) {
        // set the rendering pipeline
        let render_object = self.uniform_shader.render_object();
        pass.set_pipeline(&render_object.pipeline);
        match &render_object.bind_group {
            None => pass.set_bind_group(0, None, &[]),
            Some(bind) => pass.set_bind_group(0, Some(bind), &[])
        }

        // render with pipeline
        for shape in &self.shape_objects {
            pass.set_vertex_buffer(0, shape.vertexes.slice(..));
            for object in &shape.objects {
                pass.set_index_buffer(object.indexes.slice(..), IndexFormat::Uint32);
                pass.draw(0..object.indexes.size() as u32, 0..1);
            }
        }
    }
}