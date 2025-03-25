pub mod uniform_shader;
pub mod object;

use crate::output::object::{ObjectShader, ObjectClass};
use crate::output::uniform_shader::UniformShader;
use std::fmt::Debug;
use wgpu::{BindGroup, Device, IndexFormat, RenderPass, RenderPipeline, TextureFormat};
use wgpu::util::RenderEncoder;

#[derive(Debug)]
struct RenderObject {
    pipeline: RenderPipeline,
    bind_group: Option<BindGroup>
}

// pub trait Shader: Debug {
//     fn new(device: &Device, format: TextureFormat) -> Self;
//     fn render_object(&self) -> &RenderObject;
// }

#[derive(Debug)]
pub struct Output {
    /// Will be modified constantly so a resizable vector makes sense.
    /// todo: remove [`pub`].
    pub object_classes: Vec<ObjectClass>,
    uniform_shader: UniformShader
}

impl Output {
    pub fn new(device: &Device, format: TextureFormat) -> Self {
        Self {
            object_classes: Vec::new(),
            uniform_shader: UniformShader::new(device, format)
        }
    }
    
    fn load_shader(&self, pass: &mut RenderPass, shader: ObjectShader) {
        let render_object = &match shader {
            ObjectShader::Uniform(color) => &self.uniform_shader,
            _ => todo!()
        }.render_object;

        pass.set_pipeline(&render_object.pipeline);
        match &render_object.bind_group {
            None => pass.set_bind_group(0, None, &[]),
            Some(bind) => pass.set_bind_group(0, Some(bind), &[])
        }
    }
    
    pub fn render(&self, pass: &mut RenderPass) {
        for shape in &self.object_classes {
            self.load_shader(pass, shape.shader);
            
            pass.set_vertex_buffer(0, shape.vertexes.slice(..));
            for object in &shape.objects {
                pass.set_index_buffer(object.indexes.slice(..), IndexFormat::Uint32);
                pass.draw(0..object.indexes.size() as u32, 0..1);
            }
        }
    }
}