use egui_wgpu::ScreenDescriptor;
use egui_wgpu::wgpu;
use egui_wgpu::wgpu::TextureViewDescriptor;
use egui_wgpu::wgpu::util::DeviceExt;
use egui_winit::winit;
use egui_winit::winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};
use std::collections::VecDeque;
use std::sync::Arc;

mod camera;
mod keybind;
mod texture;
mod transform;
mod util;
mod vertex_handler;
use keybind::{CameraKeybind, KeybindHandler, TransformKeybind};
use util::Cycle;
mod gui;
mod input_controller;
use crate::{camera::CameraController, transform::TransformController};

mod gui_example;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// A single instance of vertex buffer with indices
struct VertBufferItem {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

// This will store the state of our game
pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    mouse_position: PhysicalPosition<f64>,
    window: Arc<Window>,

    // Rendering pipeline (Used for implementing shaders)
    pipeline_deque: VecDeque<wgpu::RenderPipeline>,
    vert_buffer_deque: VecDeque<VertBufferItem>,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,

    camera_object: camera::Camera,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_controller: camera::CameraController,

    transform_object: transform::Transform,
    transform_uniform: transform::TransformUniform,
    transform_buffer: wgpu::Buffer,
    transform_bind_group: wgpu::BindGroup,
    transform_controller: transform::TransformController,

    egui_renderer: gui::EguiRenderer,
    gui_state: Box<dyn gui::GuiState>,
}

impl State {
    // We don't need this to be async right now,
    // but we will in the next tutorial
    pub async fn new(window: Arc<Window>) -> anyhow::Result<State> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone())?;

        //Handle for gpu, used to create device and queue later
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // If the above request adapter does not retrieve a suitable adapter, use code below
        // let adapter = instance
        //     .enumerate_adapters(wgpu::Backends::all())
        //     .filter(|adapter| {
        //         // Check if this adapter supports our surface
        //         adapter.is_surface_supported(&surface)
        //     })
        //     .next()
        //     .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                // adapter.features() or device.features() => Show all features of the GPU used
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            // How SurfaceTexture will be used
            format: surface_format,
            // How SurfaceTexture will be stored on GPU
            width: size.width,   // >0
            height: size.height, // >0
            present_mode: surface_caps.present_modes[0],
            // How surface is synced to display
            // VSYNC: => PresentMode::Fifo (Cap display rate to framerate)
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            // List of texture formats used when creating texture views
            // I.e surface is sRGB => texture view created with linear color space
            desired_maximum_frame_latency: 2,
        };

        // TEXTURE MAKING
        let diffuse_texture =
            texture::Texture::from_bytes(&device, &queue, "src/info/happy_tree.png", "happy tree")?;

        // Bind group layout: Master bind group with which other bind groups can extend from
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        // Bind group: Describes a set of resources and how they are accessed by the shader
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        // TRANSFORM -----------------------------------
        let transform_object = transform::Transform::default();

        let mut transform_uniform = transform::TransformUniform::new();
        transform_uniform.update_matrix(&transform_object);

        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&[transform_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let transform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX, // Only visible to vertex shader
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false, // Location of the data in the transform_buffer may change (if true)
                        min_binding_size: None,    // Smallest buffer size
                    },
                    count: None,
                }],
                label: Some("transform_bind_group_layout"),
            });

        let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &transform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding(),
            }],
            label: Some("transform_bind_group"),
        });

        let transform_controller = TransformController::new();

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

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX, // Only visible to vertex shader
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false, // Location of the data in the camera_buffer may change (if true)
                        min_binding_size: None,    // Smallest buffer size
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let camera_controller = CameraController::new(0.1);

        // Loading shaders
        // Shader shortcut for this
        // let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        //     label: Some("Shader"),
        //     source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        // });
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        // Pipeline layout
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &camera_bind_group_layout,
                    &transform_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        // Main render pipeline
        let _main_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),

            // Vertex shader
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // Vertex shader entry point (Functions with @vertex and @fragment)
                buffers: &[vertex_handler::Vertex::desc()], // Specify what types of vertices are passed onto the vertex shader
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },

            // Fragment sjader
            fragment: Some(wgpu::FragmentState {
                // Fragment is technically optional, needed to store color data to surface
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    // Tells Wgpu what color outputs to set up
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL, // Write to rgba
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),

            // Tells gpu how to interpret vertices when converting them to triangles
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // Every three vertices correspond to a triangle
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // Triangle is facing forward if vertices are arranged in counter clockwise direction
                cull_mode: Some(wgpu::Face::Back), // Triangles are culled if not facing forward
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,                         // How many samples the pipeline will used (Too much work)
                mask: !0, // Mask specifies which samples should be active (All of them)
                alpha_to_coverage_enabled: false, // Anti aliasing
            },
            multiview: None, // How many layers the render attachments will have (For array textures)
            cache: None, // Cache shader compliation data (Only useful for android build targets)
        });

        // Challenge 3 render pipeline
        let _challenge3_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),

            // Vertex shader
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"), // Vertex shader entry point (Functions with @vertex and @fragment)
                buffers: &[vertex_handler::Vertex::desc()], // Specify what types of vertices are passed onto the vertex shader
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },

            // Fragment shader
            fragment: Some(wgpu::FragmentState {
                // Fragment is technically optional, needed to store color data to surface
                module: &shader,
                entry_point: Some("fs_texture"),
                targets: &[Some(wgpu::ColorTargetState {
                    // Tells Wgpu what color outputs to set up
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE), // Replace old pixel with new pixel
                    write_mask: wgpu::ColorWrites::ALL,     // Write to rgba
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),

            // Tells gpu how to interpret vertices when converting them to triangles
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,                         // How many samples the pipeline will used (Too much work)
                mask: !0, // Mask specifies which samples should be active (All of them)
                alpha_to_coverage_enabled: false, // Anti aliasing
            },
            multiview: None, // How many layers the render attachments will have (For array textures)
            cache: None, // Cache shader compliation data (Only useful for android build targets)
        });

        let pipeline_deque = VecDeque::from([_main_pipeline, _challenge3_pipeline]);

        // Create vertices from json file
        let vertex_json_container = vertex_handler::parse_json("src/info/vertices_tex.json")?;
        let vert_buffer_deque = VecDeque::from(
            vertex_json_container
                .into_iter()
                .map(|shape| State::create_buffer_item(&device, &shape.vertices, &shape.indices))
                .collect::<Vec<_>>(),
        );

        let egui_renderer = gui::EguiRenderer::new(
            &device,       // wgpu Device
            config.format, // TextureFormat
            None,          // this can be None
            1,             // samples
            &window,       // winit Window
        );

        // UI variables stored here
        let gui_state = Box::new(gui_example::GuiExample::default());

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            mouse_position: PhysicalPosition::default(),
            pipeline_deque,
            vert_buffer_deque,
            window,

            diffuse_bind_group,
            diffuse_texture,

            camera_object,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            camera_controller,

            transform_object,
            transform_buffer,
            transform_uniform,
            transform_bind_group,
            transform_controller,

            egui_renderer,
            gui_state,
        })
    }

    // Supports resizing application
    // Surface is reconfigured everytime the window size changes
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    fn create_buffer_item(
        device: &wgpu::Device,
        vertices: &[vertex_handler::Vertex],
        indices: &[u16],
    ) -> VertBufferItem {
        // Vertex buffer initialistion
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices), // Cast to &[u8]
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = indices.len() as u32;

        VertBufferItem {
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    // Handles keyboard events
    fn handle_key(
        &mut self,
        event_loop: &ActiveEventLoop,
        code: KeyCode,
        is_pressed: bool,
    ) -> anyhow::Result<()> {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            (KeyCode::Space, true) => self.switch_render()?,
            (KeyCode::KeyY, true) => self.switch_shape()?,
            (KeyCode::KeyW | KeyCode::ArrowUp, _) => {
                self.camera_controller
                    .handle_keybind(CameraKeybind::Forward, is_pressed)?;
            }
            (KeyCode::KeyA | KeyCode::ArrowLeft, _) => {
                self.camera_controller
                    .handle_keybind(CameraKeybind::Left, is_pressed)?;
            }
            (KeyCode::KeyS | KeyCode::ArrowDown, _) => {
                self.camera_controller
                    .handle_keybind(CameraKeybind::Backward, is_pressed)?;
            }
            (KeyCode::KeyD | KeyCode::ArrowRight, _) => {
                self.camera_controller
                    .handle_keybind(CameraKeybind::Right, is_pressed)?;
            }
            (KeyCode::KeyI, _) => {
                self.transform_controller
                    .handle_keybind(TransformKeybind::RotateX, is_pressed)?;
            }
            (KeyCode::KeyO, _) => {
                self.transform_controller
                    .handle_keybind(TransformKeybind::RotateY, is_pressed)?;
            }
            (KeyCode::KeyP, _) => {
                self.transform_controller
                    .handle_keybind(TransformKeybind::RotateZ, is_pressed)?;
            }
            (KeyCode::KeyK, _) => {
                self.transform_controller
                    .handle_keybind(TransformKeybind::TranslateX, is_pressed)?;
            }
            (KeyCode::KeyL, _) => {
                self.transform_controller
                    .handle_keybind(TransformKeybind::TranslateY, is_pressed)?;
            }
            (KeyCode::Semicolon, _) => {
                self.transform_controller
                    .handle_keybind(TransformKeybind::TranslateZ, is_pressed)?;
            }
            (KeyCode::Comma, _) => {
                self.transform_controller
                    .handle_keybind(TransformKeybind::ScaleX, is_pressed)?;
            }
            (KeyCode::Period, _) => {
                self.transform_controller
                    .handle_keybind(TransformKeybind::ScaleY, is_pressed)?;
            }
            (KeyCode::Slash, _) => {
                self.transform_controller
                    .handle_keybind(TransformKeybind::ScaleZ, is_pressed)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn switch_shape(&mut self) -> anyhow::Result<()> {
        self.vert_buffer_deque.cycle()?;
        Ok(())
    }

    fn switch_render(&mut self) -> anyhow::Result<()> {
        self.pipeline_deque.cycle()?;
        Ok(())
    }

    fn handle_mouse_position(&mut self, position: &mut PhysicalPosition<f64>) {
        self.mouse_position = *position;
    }

    fn update(&mut self) {
        self.camera_controller
            .update_camera(&mut self.camera_object);
        self.camera_uniform.update_view_proj(&self.camera_object);

        self.transform_controller
            .update_transform(&mut self.transform_object);
        self.transform_uniform.update_matrix(&self.transform_object);

        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        self.queue.write_buffer(
            &self.transform_buffer,
            0,
            bytemuck::cast_slice(&[self.transform_uniform]),
        );
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        // Wait for surface to provide a new surface that will be rendered
        let output = self.surface.get_current_texture()?;

        // Creates texture view with default settings
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Builds a command buffer to send to the GPU
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Drawing to the screen
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,          // What view to save to
                resolve_target: None, // Texture that will recieve the resolved output
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: self.mouse_position.x / self.config.width as f64,
                        g: self.mouse_position.y / self.config.height as f64,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        // Load render pipeline
        render_pass.set_pipeline(
            self.pipeline_deque
                .front()
                .ok_or(wgpu::SurfaceError::OutOfMemory)?,
        );

        // Load bind group
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
        render_pass.set_bind_group(2, &self.transform_bind_group, &[]);

        // Get front most vertex buffer item
        let front_vertbuf_item = self
            .vert_buffer_deque
            .front()
            .ok_or(wgpu::SurfaceError::OutOfMemory)?;

        // Load data for vertex buffer
        render_pass.set_vertex_buffer(0, front_vertbuf_item.vertex_buffer.slice(..));

        // Index buffer (CAN ONLY HAVE ONE SET AT A TIME)
        render_pass.set_index_buffer(
            front_vertbuf_item.index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );

        // Create "Something" with the given number of vertices and indices
        // Regular draw() ignores indices
        render_pass.draw_indexed(0..front_vertbuf_item.num_indices, 0, 0..1);

        // Render pass mutably borrows encoder
        // in order to continue using encoder, this borrow needs to be released
        drop(render_pass);

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: self.window().scale_factor() as f32,
        };

        // EGUI
        self.egui_renderer.draw(
            &self.device,
            &self.queue,
            &mut encoder,
            &self.window,
            &view,
            screen_descriptor,
            &mut |ui| self.gui_state.show_gui(ui),
        );

        // Submit to GPUs render queue
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

#[derive(Default)]
pub struct App {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    state: Option<State>,
}

impl App {
    pub fn new(#[cfg(target_arch = "wasm32")] event_loop: &EventLoop<State>) -> Self {
        #[cfg(target_arch = "wasm32")]
        let proxy = Some(event_loop.create_proxy());
        Self {
            state: None,
            #[cfg(target_arch = "wasm32")]
            proxy,
        }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            const CANVAS_ID: &str = "canvas";

            let window = wgpu::web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();
            let html_canvas_element = canvas.unchecked_into();
            window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
        }

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            // If we are not on web we can use pollster to
            // await the
            self.state = Some(pollster::block_on(State::new(window)).unwrap());
        }

        #[cfg(target_arch = "wasm32")]
        {
            // Run the future asynchronously and use the
            // proxy to send the results to the event loop
            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                    assert!(
                        proxy
                            .send_event(
                                State::new(window)
                                    .await
                                    .expect("Unable to create canvas!!!")
                            )
                            .is_ok()
                    )
                });
            }
        }
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        #[cfg(target_arch = "wasm32")]
        {
            event.window.request_redraw();
            event.resize(
                event.window.inner_size().width,
                event.window.inner_size().height,
            );
        }
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
            }
            WindowEvent::CursorMoved { mut position, .. } => {
                state.handle_mouse_position(&mut position)
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                if let Err(e) = state.handle_key(event_loop, code, key_state.is_pressed()) {
                    log::error!("Error key input! {}", e);
                }
            }
            _ => {}
        }

        state.egui_renderer.handle_input(&state.window, &event);
        state.update();
    }
}

pub fn run() -> anyhow::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(log::Level::Info).unwrap_throw();
    }

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new(
        #[cfg(target_arch = "wasm32")]
        &event_loop,
    );
    event_loop.run_app(&mut app)?;

    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    run().unwrap_throw();

    Ok(())
}
