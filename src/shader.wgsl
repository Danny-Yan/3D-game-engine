// Vertex shader
// Controls the individual vertices of the shape generated
struct VertexOutput {
    // Framebuffer space => max(x and y) == size of window
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;

    // Triangle
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.vert_pos = out.clip_position.xyz;
    return out;
}

// Fragment shader
// Controls each "fragment" => basically pixel
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Brown
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}

@fragment
fn fs_challenge3(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.clip_position;
}