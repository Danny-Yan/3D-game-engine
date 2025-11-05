mod backend;
#[rustfmt::skip]
use crate::backend::renderer::{
    bind_group_objects, 
    camera, 
    controller::{ControllerUniform},
    gui, 
    gui_version1, 
    instance, 
    keybind,
    keybind::{KeybindHandler, Keybind, AppKeybind},
    lighting, 
    model,
    model::DrawModel,
    pipeline, 
    resources, 
    texture, 
    transform, 
    util,
    util::Cycle,
};
use cgmath::prelude::*;
use egui_wgpu::ScreenDescriptor;
use egui_wgpu::wgpu;
use egui_winit::winit;
use egui_winit::winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorGrabMode, Window},
};
use std::collections::VecDeque;
use std::sync::Arc;
use std::{thread, time::Duration};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

struct World {
    instances_list: Vec<(String, Vec<cgmath::Vector3<f32>>)>,
}

impl World {
    fn new() -> Self {
        Self {
            // Loading models
            instances_list: vec![
                (
                    "chinese_war_ship.obj".to_string(),
                    (0..instance::NUM_INSTANCES_PER_ROW)
                        .flat_map(|z| {
                            (0..instance::NUM_INSTANCES_PER_ROW).map(move |x| {
                                let x = instance::SPACE_BETWEEN
                                    * (x as f32 - instance::NUM_INSTANCES_PER_ROW as f32 / 2.0);
                                let z = instance::SPACE_BETWEEN
                                    * (z as f32 - instance::NUM_INSTANCES_PER_ROW as f32 / 2.0);
                                cgmath::Vector3::new(x, 0.0, z)
                            })
                        })
                        .collect::<Vec<_>>(),
                ),
                (
                    "Car.obj".to_string(),
                    (0..instance::NUM_INSTANCES_PER_ROW)
                        .flat_map(|z| {
                            (0..instance::NUM_INSTANCES_PER_ROW).map(move |x| {
                                let x = instance::SPACE_BETWEEN
                                    * (x as f32 - instance::NUM_INSTANCES_PER_ROW as f32 / 2.0);
                                let z = instance::SPACE_BETWEEN
                                    * (z as f32 - instance::NUM_INSTANCES_PER_ROW as f32 / 2.0);
                                cgmath::Vector3::new(x, 5.0, z)
                            })
                        })
                        .collect::<Vec<_>>(),
                ),
            ],
        }
    }

    fn update(&mut self) {
        // for object in &mut self.objects {
        //     for model_instance in &mut object.0 {
        //         for instance in &mut model_instance.instances {
        //             let amount = cgmath::Quaternion::from_angle_y(cgmath::Rad(0.01));
        //             let current = instance.rotation;
        //             instance.rotation = amount * current;
        //         }
        //     }
        // }
    }
}

const ROTATION_SPEED: f32 = 2.0 * std::f32::consts::PI / 60.0;
// This will store the state of our game
// NO REFERENCES EVER BAD VERY BAD NO GOOD
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

    // Bind groups
    bind_group_layouts: bind_group_objects::BindGroupLayouts,
    bind_obj: bind_group_objects::GPUBindGroupObjects,
    bind_groups: bind_group_objects::BindGroups,

    world: World,
    models_instance_list: model::ModelInstanceList,
    engine_rendered_models_list: model::ModelInstanceList,
    egui_renderer: gui::EguiRenderer,

    is_rotation: bool,
    is_fps_camera: bool,
    input_mapper: keybind::InputMapper,
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

        // Adding bind groups
        let bind_group_layouts = bind_group_objects::BindGroupLayouts::new(&device);
        let bind_group_objects =
            bind_group_objects::GPUBindGroupObjects::new(&device, &queue, &config);
        let bind_groups =
            bind_group_objects::BindGroups::new(&device, &bind_group_layouts, &bind_group_objects);

        // Loading world
        let world = World::new();
        let mut models_instance_list = model::ModelInstanceList::new();
        // Add models by name and positions
        for (name, positions) in world.instances_list.clone() {
            models_instance_list
                .add_instances(
                    &name,
                    positions,
                    &device,
                    &queue,
                    &bind_group_layouts.texture,
                )
                .await?;
        }

        let engine_rendered_models_list = model::ModelInstanceList::new();

        // Pipeline
        let pipeline_layouts = pipeline::PipelineLayouts::new(
            &device,
            &[
                &bind_group_layouts.texture,
                &bind_group_layouts.uniform,
                &bind_group_layouts.depth,
                &bind_group_layouts.light,
            ],
        );
        let pipeline_deque =
            pipeline::Pipelines::new(&device, &config, &pipeline_layouts).convert_to_deque();

        let egui_renderer = gui::EguiRenderer::new(
            &device,       // wgpu Device
            config.format, // TextureFormat
            None,          // this can be None
            1,             // samples
            &window,       // winit Window
        );

        let input_mapper = keybind::InputMapper::new();

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            mouse_position: PhysicalPosition::default(),
            pipeline_deque,
            window,
            bind_group_layouts,
            bind_obj: bind_group_objects,
            bind_groups,

            world,
            models_instance_list,
            engine_rendered_models_list,

            egui_renderer,

            // Probs need to make into yaml or smth
            is_rotation: false,
            is_fps_camera: false,
            input_mapper,
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

            // Update depth texture so it always has the same size as the window
            self.bind_obj.depth.texture =
                texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
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
        let current_keybind = self.input_mapper.handle_key(code);
        let camera_controller = &mut self.bind_obj.camera.controller;
        let transform_controller = &mut self.bind_obj.transform.controller;

        if let Some(keybind) = current_keybind {
            match keybind {
                Keybind::App(app_keybind) => {
                    if !is_pressed {
                        return Ok(());
                    }
                    match app_keybind {
                        AppKeybind::Exit => event_loop.exit(),
                        AppKeybind::SwitchRender => self.switch_render()?,
                        AppKeybind::SwitchScene => self.switch_shape()?,
                        AppKeybind::ToggleRotationMode => self.is_rotation = !self.is_rotation,
                        AppKeybind::ToggleCameraMode => {
                            self.switch_input_context()?;
                        }
                    }
                }
                Keybind::Camera(camera_keybind) => camera_controller
                    .front_mut()
                    .ok_or(wgpu::SurfaceError::OutOfMemory)?
                    .handle_keybind(camera_keybind, is_pressed)?,
                Keybind::Transform(transform_keybind) => {
                    transform_controller.handle_keybind(transform_keybind, is_pressed)?
                }
            }
        };

        Ok(())
    }

    fn switch_shape(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn switch_render(&mut self) -> anyhow::Result<()> {
        self.pipeline_deque.cycle()?;
        Ok(())
    }

    fn switch_input_context(&mut self) -> anyhow::Result<()> {
        self.input_mapper.cycle_contexts();
        self.switch_camera()
    }

    fn switch_camera(&mut self) -> anyhow::Result<()> {
        self.is_fps_camera = !self.is_fps_camera;
        self.bind_obj.camera.controller.cycle()?;

        if self.is_fps_camera {
            self.window.set_cursor_grab(CursorGrabMode::Locked)?
        } else {
            self.window.set_cursor_grab(CursorGrabMode::None)?
        }

        self.bind_obj
            .camera
            .controller
            .front_mut_err()?
            .update_controller(&self.bind_obj.camera.object);

        Ok(())
    }

    fn handle_mouse_position(
        &mut self,
        position: &mut PhysicalPosition<f64>,
    ) -> anyhow::Result<()> {
        self.mouse_position = *position;
        self.bind_obj
            .camera
            .controller
            .front_mut_err()?
            .update_model_mouse(
                *position,
                PhysicalPosition {
                    x: self.config.width as f64 / 2.0,
                    y: self.config.height as f64 / 2.0,
                },
            );

        Ok(())
    }

    // Tick method
    fn update(&mut self, delta_time: f32, event_loop: &ActiveEventLoop) -> anyhow::Result<()> {
        // Rotate light
        // let old_position: cgmath::Vector3<_> = self.light_uniform.position.into();
        // self.light_uniform.position =
        //     (cgmath::Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), cgmath::Deg(1.0))
        //         * old_position)
        //         .into();
        // self.queue.write_buffer(
        //     &self.light_buffer,
        //     0,
        //     bytemuck::cast_slice(&[self.light_uniform]),
        // );
        if self.is_fps_camera {
            // Centre camera
            self.window.set_cursor_grab(CursorGrabMode::Locked)?;

            thread::sleep(Duration::from_millis(delta_time as u64));
            self.window
                .set_cursor_position(winit::dpi::PhysicalPosition::new(
                    self.config.width as f64 / 2.0,
                    self.config.height as f64 / 2.0,
                ))?;

            self.window.set_cursor_grab(CursorGrabMode::None)?;
        }

        // Camera
        self.bind_obj
            .camera
            .controller
            .front_mut()
            .ok_or(wgpu::SurfaceError::OutOfMemory)?
            .update_model(&mut self.bind_obj.camera.object);
        self.bind_obj
            .camera
            .uniform
            .update_matrix(&self.bind_obj.camera.object);

        self.queue.write_buffer(
            &self.bind_obj.camera.buffer,
            0,
            bytemuck::cast_slice(&[self.bind_obj.camera.uniform]),
        );

        // Transform
        self.bind_obj
            .transform
            .uniform
            .update_matrix(&self.bind_obj.transform.object);

        self.queue.write_buffer(
            &self.bind_obj.transform.buffer,
            0,
            bytemuck::cast_slice(&[self.bind_obj.transform.uniform]),
        );

        // World
        self.world.update();

        // Update instances
        for model_instance in &mut self.models_instance_list.0 {
            // ROTATE INSTANCES

            // Change to be more ergonomic (too many if statements)
            if self.is_rotation {
                for instance in &mut model_instance.instances {
                    let rotation_amount = ROTATION_SPEED;
                    let amount = cgmath::Quaternion::from_angle_y(cgmath::Rad(rotation_amount));
                    let current = instance.rotation;
                    instance.rotation = amount * current;
                }
            }

            // INSTANCE UPDATED HERE
            let instance_data = model_instance
                .instances
                .iter()
                .map(instance::Instance::to_raw)
                .collect::<Vec<_>>();
            self.queue.write_buffer(
                &model_instance.instance_buffer,
                0,
                bytemuck::cast_slice(&instance_data),
            );
        }
        Ok(())
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
            // Attaches the depth buffer and stencil buffer (Part of the same buffer)
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.bind_obj.depth.texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        // Load render pipeline
        render_pass.set_pipeline(
            // ??????????????????????????????????????????????
            // Why can't I change this to front_err()
            self.pipeline_deque
                .front()
                .ok_or(wgpu::SurfaceError::OutOfMemory)?,
        );

        // Load bind group
        render_pass.set_bind_group(0, &self.bind_groups.texture, &[]);
        render_pass.set_bind_group(1, &self.bind_groups.uniform, &[]);
        render_pass.set_bind_group(2, &self.bind_groups.depth, &[]);
        render_pass.set_bind_group(3, &self.bind_groups.light, &[]);

        for model_instance in &self.models_instance_list.0 {
            render_pass.set_vertex_buffer(1, model_instance.instance_buffer.slice(..));
            render_pass.draw_model_instanced(
                &model_instance.model,
                0..model_instance.instances.len() as u32,
                &self.bind_groups.uniform,
            );
        }
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
            &mut |ui| {
                gui_version1::show_gui(
                    ui,
                    &mut gui_version1::GuiOptions::default(),
                    // Manual copying of state fields
                    &mut gui_version1::GuiState {
                        models_instance_list: &mut self.models_instance_list,
                        transform_object: &mut self.bind_obj.transform.object,
                        window: &self.window,
                        device: &self.device,
                        queue: &self.queue,
                        texture_bind_group_layout: &self.bind_group_layouts.texture,
                    },
                )
            },
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
        // window_attributes.inner_size = Some(winit::dpi::Size::Logical(
        //     winit::dpi::LogicalSize::new(800.0, 600.0),
        // ));
        window_attributes.maximized = true;
        window_attributes.title = "Epic window".to_owned();

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
            Some(s) => s,
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
                if let Err(e) = state.handle_mouse_position(&mut position) {
                    log::error!("Mouse error {}", e);
                }
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
                    log::error!("Incorrect key press {}", e);
                };
            }
            _ => {}
        }

        state.egui_renderer.handle_input(&state.window, &event);
        if let Err(e) = state.update(16.67, event_loop) {
            log::error!("Update error {}", e);
        }; // Temporary hard coded tick speed
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
