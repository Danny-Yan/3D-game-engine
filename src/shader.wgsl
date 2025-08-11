// Vertex shader
// Controls the individual vertices of the shape generated

// struct VertexInput {
//     @location(0) position: vec3<f32>,
//     @location(1) color: vec3<f32>,
// };

// struct VertexOutput {
//     // Framebuffer space
//     // Position of shape in relation to the camera (Measured as per pixel)
//     // MAX (x and y) == size of window
//     @builtin(position) framebuffer_position: vec4<f32>,

//     // Color of vertex
//     @location(0) color: vec3<f32>,
// };

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) framebuffer_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    // Triangle
    var out: VertexOutput;
    out.tex_coords  = model.tex_coords;
    out.framebuffer_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader
// Controls each "fragment" => basically pixel
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Brown
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}

// @fragment
// fn fs_challenge3(in: VertexOutput) -> @location(0) vec4<f32> {
//     return vec4<f32>(in.color, 1.0);
// }

// From diffuse bind group
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_texture(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
 