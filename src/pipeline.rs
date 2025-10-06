use crate::bind_group::BindGroupLayouts;
use crate::{instance, model, model::Vertex, texture};
use std::collections::VecDeque;

pub struct PipelineLayouts {
    pub render: wgpu::PipelineLayout,
}

impl PipelineLayouts {
    pub fn new(device: &wgpu::Device, bind_group_layouts: &BindGroupLayouts) -> Self {
        // Pipeline layout
        let render = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &bind_group_layouts.as_array(),
            push_constant_ranges: &[],
        });
        Self { render }
    }
}

pub struct Pipelines {
    _depth_pipeline: wgpu::RenderPipeline,
    _main_pipeline: wgpu::RenderPipeline,
}

impl Pipelines {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        pipeline_layouts: &PipelineLayouts,
    ) -> Self {
        // Depth pipeline
        let _depth_pipeline = {
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Normal Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            };
            Pipelines::create_render_pipeline(
                device,
                &pipeline_layouts.render,
                Some("vs_main"),
                Some("fs_depth_texture"),
                config.format,
                Some(texture::Texture::DEPTH_FORMAT),
                &[model::ModelVertex::desc(), instance::InstanceRaw::desc()],
                shader,
            )
        };

        // Main render pipeline
        let _main_pipeline = {
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Normal Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            };
            Pipelines::create_render_pipeline(
                device,
                &pipeline_layouts.render,
                Some("vs_main"),
                Some("fs_texture"),
                config.format,
                Some(texture::Texture::DEPTH_FORMAT),
                &[model::ModelVertex::desc(), instance::InstanceRaw::desc()],
                shader,
            )
        };

        Self {
            _depth_pipeline,
            _main_pipeline,
        }
    }

    pub fn as_deque(&self) -> VecDeque<&wgpu::RenderPipeline> {
        VecDeque::from([&self._depth_pipeline, &self._main_pipeline])
    }

    fn create_render_pipeline(
        device: &wgpu::Device,
        layout: &wgpu::PipelineLayout,
        vertex_entry: Option<&str>,
        fragment_entry: Option<&str>,
        color_format: wgpu::TextureFormat,
        depth_format: Option<wgpu::TextureFormat>,
        vertex_layouts: &[wgpu::VertexBufferLayout],
        shader: wgpu::ShaderModuleDescriptor,
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(shader);

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(layout),

            // VERTEX SHADER
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: vertex_entry, // Vertex shader entry point (Functions with @vertex and @fragment)
                buffers: vertex_layouts, // Specify what types of vertices are passed onto the vertex shader
                compilation_options: Default::default(),
            },

            // FRAGMENT SHADER
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: fragment_entry,
                targets: &[Some(wgpu::ColorTargetState {
                    // Tells Wgpu what color outputs to set up
                    format: color_format,
                    blend: Some(wgpu::BlendState {
                        alpha: wgpu::BlendComponent::REPLACE,
                        color: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL, // Write to rgba
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // Every three vertices correspond to a triangle
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // Triangle is facing forward if vertices are arranged in counter clockwise direction
                cull_mode: Some(wgpu::Face::Back), // Triangles are culled if not facing forward
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            // DEPTH BUFFER
            depth_stencil: depth_format.map(|format| wgpu::DepthStencilState {
                format,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // How pixels are compared in the depth (Less means front to back)
                stencil: wgpu::StencilState::default(), // Stores a mask or pattern that will cut out certain areas of the drawing
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,                         // How many samples the pipeline will used (Too much work)
                mask: !0, // Mask specifies which samples should be active (All of them)
                alpha_to_coverage_enabled: false, // Anti aliasing
            },
            multiview: None, // How many layers the render attachments will have (For array textures)
            cache: None, // Cache shader compliation data (Only useful for android build targets)
        })
    }
}
