use std::f32::consts::E;
use std::rc::Rc;
use std::{default, sync::Arc};
use wgpu::wgc::device;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    event::*,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use std::collections::VecDeque;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// This will store the state of our game
pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    position: PhysicalPosition<f64>,
    window: Arc<Window>,

    // Rendering pipeline (Used for implementing shaders)
    pipeline_deque: VecDeque<wgpu::RenderPipeline>,
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

        let surface = instance.create_surface(window.clone()).unwrap();

        //Handle for gpu, used to create device and queue later
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

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
                bind_group_layouts: &[],
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
                buffers: &[], // Specify what types of vertices are passed onto the vertex shader
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
                buffers: &[], // Specify what types of vertices are passed onto the vertex shader
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },

            // Fragment shader
            fragment: Some(wgpu::FragmentState {
                // Fragment is technically optional, needed to store color data to surface
                module: &shader,
                entry_point: Some("fs_challenge3"),
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

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            position: PhysicalPosition::default(),
            pipeline_deque: pipeline_deque,
            window,
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
            _ => {}
        }
        Ok(())
    }

    fn switch_render(&mut self) -> anyhow::Result<()> {
        let front_item = self
            .pipeline_deque
            .pop_front()
            .ok_or(wgpu::SurfaceError::OutOfMemory)?;
        self.pipeline_deque.push_back(front_item);
        Ok(())
    }

    fn handle_mouse_position(&mut self, position: &mut PhysicalPosition<f64>) {
        self.position = *position;
    }

    fn update(&mut self) {
        // remove `todo!()`
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
                        r: self.position.x / self.config.width as f64,
                        g: self.position.y / self.config.height as f64,
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

        render_pass.set_pipeline(
            self.pipeline_deque
                .front()
                .ok_or(wgpu::SurfaceError::OutOfMemory)?,
        ); // Load render pipeline
        render_pass.draw(0..3, 0..1); // Create "Something" with 3 vertices and 1 instance

        // Render pass mutably borrows encoder
        // in order to continue using encoder, this borrow needs to be released
        drop(render_pass);

        // Submit to GPUs render queue
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

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
