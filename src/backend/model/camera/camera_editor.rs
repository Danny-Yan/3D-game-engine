use crate::backend::model::camera::camera_model::Camera;
use crate::backend::renderer::controller::{
    CameraControllerTrait, Controller, MouseHandler, SceneCursorMode,
};
use crate::keybind::{CameraKeybind, KeybindHandler};
use winit::window::CursorGrabMode;

pub struct CameraControllerEditor {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl Default for CameraControllerEditor {
    fn default() -> Self {
        Self {
            speed: 0.1,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }
}

impl Controller<Camera> for CameraControllerEditor {
    fn update_model(&self, camera: &mut Camera) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when the camera gets too close to the
        // center of the scene.
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the forward/backward is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_left_pressed {
            // Rescale the distance between the target and the eye so
            // that it doesn't change. The eye, therefore, still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_right_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }

    fn change_scene(&self) -> SceneCursorMode {
        SceneCursorMode {
            cursor_mode: CursorGrabMode::None,
            visible: false,
        }
    }
    // fn update_controller(&mut self, _: &Camera) {}
}

impl KeybindHandler<CameraKeybind> for CameraControllerEditor {
    fn handle_keybind(&mut self, keycode: &CameraKeybind, is_pressed: bool) -> anyhow::Result<()> {
        // // TAHNK YOU KEVIN
        // let balls = keycode
        //     .cast_to()
        //     .downcast_ref::<CameraKeybind>()
        //     .ok_or(wgpu::wgc::validation::InputError::Missing)?;
        #[allow(unreachable_patterns)]
        match keycode {
            CameraKeybind::Forward => {
                self.is_forward_pressed = is_pressed;
                Ok(())
            }
            CameraKeybind::Left => {
                self.is_left_pressed = is_pressed;
                Ok(())
            }
            CameraKeybind::Backward => {
                self.is_backward_pressed = is_pressed;
                Ok(())
            }
            CameraKeybind::Right => {
                self.is_right_pressed = is_pressed;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

impl MouseHandler for CameraControllerEditor {}

impl CameraControllerTrait for CameraControllerEditor {}
