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

// Camera
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0) // From camera_bind_group
var<uniform> camera: CameraUniform;

// Transform
struct TransformUniform {
    matrix: mat4x4<f32>,
};
@group(2) @binding(0) // From transform_bind_group
var<uniform> transform: TransformUniform;

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};

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
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    // Triangle
    var out: VertexOutput;
    out.tex_coords  = model.tex_coords;
    out.framebuffer_position = 
        camera.view_proj * 
        transform.matrix * // World space
        model_matrix *
        // transform.matrix * // Object space
        vec4<f32>(model.position, 1.0);
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


// From diffuse bind group
@group(3) @binding(0)
var t_depth: texture_depth_2d;
@group(3) @binding(1)
var s_depth: sampler_comparison;

@fragment
fn fs_depth_texture(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1, 1, 1, textureSampleCompare(t_depth, s_depth, in.tex_coords, 0.0));
}