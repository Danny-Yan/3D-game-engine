use crate::camera;
use crate::lighting;
use crate::texture;
use crate::transform;
use egui_wgpu::wgpu;
use egui_wgpu::wgpu::util::DeviceExt;

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
}

pub struct BindGroups {
    pub texture: wgpu::BindGroup,
    pub uniform: wgpu::BindGroup,
    pub depth: wgpu::BindGroup,
    pub light: wgpu::BindGroup,
}

impl BindGroups {
    pub fn new(
        device: &wgpu::Device,
        bind_group_layouts: &BindGroupLayouts,
        obj: &GPUBindGroupObjects,
    ) -> Self {
        // Bind group: Describes a set of resources and how they are accessed by the shader
        let texture = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layouts.texture,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&obj.diffuse.texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&obj.diffuse.texture.sampler),
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
                    resource: wgpu::BindingResource::TextureView(&obj.depth.texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&obj.depth.texture.sampler),
                },
            ],
            label: Some("depth_bind_group"),
        });

        let uniform = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layouts.uniform,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: obj.camera.buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: obj.transform.buffer.as_entire_binding(),
                },
            ],
            label: Some("uniform_bind_group"),
        });

        let light = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layouts.light,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: obj.light.buffer.as_entire_binding(),
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
}

pub struct GPUBindGroupObjects {
    pub camera: CameraBindGroupObject,
    pub transform: TransformBindGroupObject,
    pub depth: DepthBindGroupObject,
    pub diffuse: TextureBindGroupObject,
    pub light: LightBindGroupObject,
}

impl GPUBindGroupObjects {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let camera = CameraBindGroupObject::new(config, device);
        let transform = TransformBindGroupObject::new(device);
        let depth = DepthBindGroupObject::new(device, config);
        let diffuse = TextureBindGroupObject::new(
            device,
            queue,
            "src/info/happy_tree.png",
            "Happy tree texture",
        );
        let light = LightBindGroupObject::new(device);

        Self {
            camera,
            transform,
            depth,
            diffuse,
            light,
        }
    }
}

pub struct CameraBindGroupObject {
    pub object: camera::Camera,
    pub uniform: camera::CameraUniform,
    pub buffer: wgpu::Buffer,
    pub controller: camera::CameraController,
}

impl CameraBindGroupObject {
    pub fn new(config: &wgpu::SurfaceConfiguration, device: &wgpu::Device) -> Self {
        // CAMERA -----------------------------------
        let object = camera::Camera {
            aspect: config.width as f32 / config.height as f32,
            ..Default::default()
        };

        let mut uniform = camera::CameraUniform::new();
        uniform.update_view_proj(&object);

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let controller = camera::CameraController::new(0.1);

        Self {
            object,
            uniform,
            buffer,
            controller,
        }
    }
}

pub struct TransformBindGroupObject {
    pub object: transform::Transform,
    pub uniform: transform::TransformUniform,
    pub buffer: wgpu::Buffer,
    pub controller: transform::TransformController,
}

impl TransformBindGroupObject {
    pub fn new(device: &wgpu::Device) -> Self {
        // TRANSFORM -----------------------------------
        let object = transform::Transform::default();

        let mut uniform = transform::TransformUniform::new();
        uniform.update_matrix(&object);

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let controller = transform::TransformController::new();

        Self {
            object,
            uniform,
            buffer,
            controller,
        }
    }
}

pub struct DepthBindGroupObject {
    pub texture: texture::Texture,
}

impl DepthBindGroupObject {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let texture = texture::Texture::create_depth_texture(device, config, "depth_texture");
        Self { texture }
    }
}

pub struct TextureBindGroupObject {
    pub texture: texture::Texture,
}

impl TextureBindGroupObject {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, path: &str, label: &str) -> Self {
        // TEXTURE MAKING
        let texture = texture::Texture::from_path(device, queue, path, label)
            .unwrap_or_else(|_| panic!("Failed to load texture at path: {}", path));
        Self { texture }
    }
}

pub struct LightBindGroupObject {
    pub uniform: lighting::LightUniform,
    pub buffer: wgpu::Buffer,
}

impl LightBindGroupObject {
    pub fn new(device: &wgpu::Device) -> Self {
        // LIGHTING -----------------------------------
        let uniform = lighting::LightUniform {
            position: [2.0, 2.0, 2.0],
            _padding: 0,
            color: [1.0, 1.0, 1.0],
            _padding2: 0,
        };

        // We'll want to update our lights position, so we use COPY_DST
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light VB"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self { uniform, buffer }
    }
}
