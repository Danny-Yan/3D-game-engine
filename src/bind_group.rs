use crate::camera;
use crate::lighting;
use crate::texture;
use crate::transform;
use egui_wgpu::wgpu;
use wgpu::wgc::device;

pub struct BindGroupLayouts {
    pub texture: wgpu::BindGroupLayout,
    pub uniform: wgpu::BindGroupLayout,
    pub depth: wgpu::BindGroupLayout,
    pub light: wgpu::BindGroupLayout,
}

impl BindGroupLayouts {
    pub fn new(device: &wgpu::Device) -> Self {
        let texture = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    // Visible only to fragment shader (NONE, VERTEX, FRAGMENT, COMPUTE)
                    visibility: wgpu::ShaderStages::FRAGMENT,

                    // A Sampled texture
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    // Visible only to fragment shader
                    visibility: wgpu::ShaderStages::FRAGMENT,

                    // A sampler
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let uniform = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("uniform_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX, // Only visible to vertex shader
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false, // Location of the data in the camera_buffer may change (if true)
                        min_binding_size: None,    // Smallest buffer size
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // Bind group layout: Master bind group with which other bind groups can extend from
        let depth = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    // Visible only to fragment shader (NONE, VERTEX, FRAGMENT, COMPUTE)
                    visibility: wgpu::ShaderStages::FRAGMENT,

                    // A Sampled texture
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Depth,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    // Visible only to fragment shader
                    visibility: wgpu::ShaderStages::FRAGMENT,

                    // A sampler
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                    count: None,
                },
            ],
            label: Some("depth_bind_group_layout"),
        });

        let light = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        });

        Self {
            texture,
            uniform,
            depth,
            light,
        }
    }

    pub fn as_array(&self) -> [&wgpu::BindGroupLayout; 4] {
        [&self.texture, &self.uniform, &self.depth, &self.light]
    }
}

pub struct CameraBindGroupObject {
    pub camera: camera::Camera,
    pub camera_uniform: camera::CameraUniform,
    pub camera_buffer: wgpu::Buffer,
    pub camera_controller: camera::CameraController,
    pub bind_group: wgpu::BindGroup,
}

pub struct TransformBindGroupObject {
    pub transform: transform::Transform,
    pub transform_uniform: transform::TransformUniform,
    pub transform_buffer: wgpu::Buffer,
    pub transform_controller: transform::TransformController,
    pub bind_group: wgpu::BindGroup,
}

pub struct DepthBindGroupObject {
    pub depth_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

pub struct TextureBindGroupObject {
    pub diffuse_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

pub struct LightBindGroupObject {
    pub light_uniform: lighting::LightUniform,
    pub light_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

pub struct UniformBufferGroups {
    // Camera
    pub camera_object: camera::Camera,
    pub camera_uniform: camera::CameraUniform,
    pub camera_buffer: wgpu::Buffer,
    pub camera_controller: camera::CameraController,

    // Transform
    pub transform_object: transform::Transform,
    pub transform_uniform: transform::TransformUniform,
    pub transform_buffer: wgpu::Buffer,
    pub transform_controller: transform::TransformController,

    // Lighting
    pub light_uniform: lighting::LightUniform,
    pub light_buffer: wgpu::Buffer,
}

impl UniformBufferGroups {
    pub fn new() {
        // CAMERA -----------------------------------
        let camera_object = camera::Camera {
            aspect: config.width as f32 / config.height as f32,
            ..Default::default()
        };

        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&camera_object);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
    }
}

pub struct TextureBufferGroups {
    pub diffuse_texture: texture::Texture,
    pub depth_texture: texture::Texture,
}

pub struct BindGroups {
    pub diffuse: wgpu::BindGroup,
    pub camera: wgpu::BindGroup,
    pub transform: wgpu::BindGroup,
    pub depth: wgpu::BindGroup,
    pub light: wgpu::BindGroup,
}

impl BindGroups {
    fn new(
        device: &wgpu::Device,
        bind_group_layouts: &BindGroupLayouts,
        texture_buffers: &TextureBufferGroups,
        uniform_buffers: &UniformBufferGroups,
    ) -> Self {
        // Bind group: Describes a set of resources and how they are accessed by the shader
        let diffuse = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layouts.texture,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        &texture_buffers.diffuse_texture.view,
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(
                        &texture_buffers.diffuse_texture.sampler,
                    ),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        // Bind group: Describes a set of resources and how they are accessed by the shader
        let depth = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layouts.depth,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        &texture_buffers.depth_texture.view,
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(
                        &texture_buffers.depth_texture.sampler,
                    ),
                },
            ],
            label: Some("depth_bind_group"),
        });

        let camera = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layouts.uniform,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffers.camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let transform = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layouts.uniform,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffers.transform_buffer.as_entire_binding(),
            }],
            label: Some("transform_bind_group"),
        });

        let light = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layouts.light,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffers.light_buffer.as_entire_binding(),
            }],
            label: None,
        });

        Self {
            diffuse,
            camera,
            transform,
            depth,
            light,
        }
    }
}
