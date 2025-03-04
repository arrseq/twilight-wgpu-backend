struct Form {
    color: vec4<f32>
}

@group(0) @binding(0)
var<uniform> form: Form;

struct VertexInput {
    @builtin(position) @location(0) position: vec3<f32>
};

@fragment
fn fs_main(in: VertexInput, @location(0) c: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(in.data, 1.0);
}