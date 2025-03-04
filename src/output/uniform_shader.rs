use wgpu::{Device, RenderPass, RenderPipeline, TextureFormat};
use crate::output::Shader;

#[derive(Debug)]
pub struct UniformShader {
    pipeline: RenderPipeline
}

impl Shader for UniformShader {
    fn new(device: &Device, format: TextureFormat) -> Self {
        todo!()
    }

    fn render(pass: RenderPass) {
        todo!()
    }
}