struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) data: f32
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
//    let x = f32(1 - i32(in_vertex_index)) * 0.5;
//    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 1.0;
    var x: f32;
    var y: f32;
    var c: f32;

    if (in_vertex_index == 0) {
        x = -1.0;
        y = 1.0;
        c = 53.4;
    } else if (in_vertex_index == 1) {
        x = 0.0;
        y = -1.0;
        c = 79.6;
    } else {
        x = 1.0;
        y = 0.0;
        c = 40.000898;
    }

    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.data = c;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var pixel: f32 = 0.0;
    if (sin(in.data) > 0.0) {
        pixel = 1.0;
    }

    return vec4<f32>(pixel, pixel, pixel, 1.0);
}