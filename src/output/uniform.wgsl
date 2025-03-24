struct Color {
    color: vec4<f32>
}

@group(0) @binding(0)
var<uniform> color: Color;

struct InputVertex {
    @location(0) position: vec2<f32>
};

struct OutputVertex {
    @builtin(position) position: vec4<f32>
}

@vertex
fn vs_main(in: InputVertex) -> OutputVertex {
    return OutputVertex(vec4<f32>(in.position, 0.0, 1.0));
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return color.color;
}