use crate::State;
use crate::keybind::{CameraKeybind, KeybindHandler, TransformKeybind};
use egui_winit::winit::{event_loop::ActiveEventLoop, keyboard::KeyCode};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct FPSCamera;

pub struct EditorCamera;

// bad implementation
trait InputMappingContext {
    fn handle_input(
        &mut self,
        event_loop: &ActiveEventLoop,
        state: Rc<RefCell<State>>,
        code: KeyCode,
        is_pressed: bool,
    ) -> anyhow::Result<()>;
}

impl InputMappingContext for FPSCamera {
    fn handle_input(
        &mut self,
        event_loop: &ActiveEventLoop,
        state: Rc<RefCell<State>>,
        code: KeyCode,
        is_pressed: bool,
    ) -> anyhow::Result<()> {
        // Handle FPS camera input
        let s = state.clone();
        let state = s.borrow();
        let camera_controller = state.bind_obj.camera.controller;
        let transform_controller = state.bind_obj.transform.controller;

        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            (KeyCode::Space, true) => state.switch_render()?,
            (KeyCode::KeyY, true) => state.switch_shape()?,
            (KeyCode::KeyR, true) => state.is_rotation = !state.is_rotation,
            (KeyCode::KeyX, true) => state.is_fps_camera = !state.is_fps_camera,

            // FIX BY CREATING SUPER CONTROLLER and [keycode enum, keybind enum] assign table
            // (PREVENT different controller from being able to access same keybinds)
            (KeyCode::KeyW | KeyCode::ArrowUp, _) => {
                camera_controller.handle_keybind(CameraKeybind::Forward, is_pressed)?;
            }
            (KeyCode::KeyA | KeyCode::ArrowLeft, _) => {
                camera_controller.handle_keybind(CameraKeybind::Left, is_pressed)?;
            }
            (KeyCode::KeyS | KeyCode::ArrowDown, _) => {
                camera_controller.handle_keybind(CameraKeybind::Backward, is_pressed)?;
            }
            (KeyCode::KeyD | KeyCode::ArrowRight, _) => {
                camera_controller.handle_keybind(CameraKeybind::Right, is_pressed)?;
            }
            (KeyCode::KeyI, _) => {
                transform_controller.handle_keybind(TransformKeybind::RotateX, is_pressed)?;
            }
            (KeyCode::KeyO, _) => {
                transform_controller.handle_keybind(TransformKeybind::RotateY, is_pressed)?;
            }
            (KeyCode::KeyP, _) => {
                transform_controller.handle_keybind(TransformKeybind::RotateZ, is_pressed)?;
            }
            (KeyCode::KeyK, _) => {
                transform_controller.handle_keybind(TransformKeybind::TranslateX, is_pressed)?;
            }
            (KeyCode::KeyL, _) => {
                transform_controller.handle_keybind(TransformKeybind::TranslateY, is_pressed)?;
            }
            (KeyCode::Semicolon, _) => {
                transform_controller.handle_keybind(TransformKeybind::TranslateZ, is_pressed)?;
            }
            (KeyCode::Comma, _) => {
                transform_controller.handle_keybind(TransformKeybind::ScaleX, is_pressed)?;
            }
            (KeyCode::Period, _) => {
                transform_controller.handle_keybind(TransformKeybind::ScaleY, is_pressed)?;
            }
            (KeyCode::Slash, _) => {
                transform_controller.handle_keybind(TransformKeybind::ScaleZ, is_pressed)?;
            }
            _ => {}
        }
        Ok(())
    }
}
impl InputMappingContext for EditorCamera {
    fn handle_input(
        &mut self,
        event_loop: &ActiveEventLoop,
        state: Rc<RefCell<State>>,
        code: KeyCode,
        is_pressed: bool,
    ) -> anyhow::Result<()> {
        let camera_controller = &mut state.bind_obj.camera.controller;
        let transform_controller = &mut state.bind_obj.transform.controller;

        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            (KeyCode::Space, true) => state.switch_render()?,
            (KeyCode::KeyY, true) => state.switch_shape()?,
            (KeyCode::KeyR, true) => state.is_rotation = !state.is_rotation,
            (KeyCode::KeyX, true) => state.is_fps_camera = !state.is_fps_camera,

            // FIX BY CREATING SUPER CONTROLLER and [keycode enum, keybind enum] assign table
            // (PREVENT different controller from being able to access same keybinds)
            (KeyCode::KeyW | KeyCode::ArrowUp, _) => {
                camera_controller.handle_keybind(CameraKeybind::Forward, is_pressed)?;
            }
            (KeyCode::KeyA | KeyCode::ArrowLeft, _) => {
                camera_controller.handle_keybind(CameraKeybind::Left, is_pressed)?;
            }
            (KeyCode::KeyS | KeyCode::ArrowDown, _) => {
                camera_controller.handle_keybind(CameraKeybind::Backward, is_pressed)?;
            }
            (KeyCode::KeyD | KeyCode::ArrowRight, _) => {
                camera_controller.handle_keybind(CameraKeybind::Right, is_pressed)?;
            }
            (KeyCode::KeyI, _) => {
                transform_controller.handle_keybind(TransformKeybind::RotateX, is_pressed)?;
            }
            (KeyCode::KeyO, _) => {
                transform_controller.handle_keybind(TransformKeybind::RotateY, is_pressed)?;
            }
            (KeyCode::KeyP, _) => {
                transform_controller.handle_keybind(TransformKeybind::RotateZ, is_pressed)?;
            }
            (KeyCode::KeyK, _) => {
                transform_controller.handle_keybind(TransformKeybind::TranslateX, is_pressed)?;
            }
            (KeyCode::KeyL, _) => {
                transform_controller.handle_keybind(TransformKeybind::TranslateY, is_pressed)?;
            }
            (KeyCode::Semicolon, _) => {
                transform_controller.handle_keybind(TransformKeybind::TranslateZ, is_pressed)?;
            }
            (KeyCode::Comma, _) => {
                transform_controller.handle_keybind(TransformKeybind::ScaleX, is_pressed)?;
            }
            (KeyCode::Period, _) => {
                transform_controller.handle_keybind(TransformKeybind::ScaleY, is_pressed)?;
            }
            (KeyCode::Slash, _) => {
                transform_controller.handle_keybind(TransformKeybind::ScaleZ, is_pressed)?;
            }
            _ => {}
        }
        Ok(())
    }
}
pub struct InputMapper {
    input_mapping_context: Box<dyn InputMappingContext>,
}

impl InputMapper {
    pub fn new() -> Self {
        Self {
            input_mapping_context: Box::new(EditorCamera {}),
        }
    }

    pub fn switch_context(&mut self, new_context: Box<dyn InputMappingContext>) {
        self.input_mapping_context = new_context;
    }

    pub fn handle_input(
        &mut self,
        event_loop: &ActiveEventLoop,
        state: Rc<RefCell<State>>,
        code: KeyCode,
        is_pressed: bool,
    ) -> anyhow::Result<()> {
        self.input_mapping_context
            .handle_input(event_loop, state, code, is_pressed)?;
        Ok(())
    }
}

//
// Key pressed -> Set keybind value here:
// Update a global truth table of keybinds

// Within state:
//
