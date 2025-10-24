use crate::backend::renderer::controller::{
    Controller, ControllerModel, ControllerUniform, MouseHandler,
};
use crate::keybind::{CameraKeybind, KeybindHandler};
use cgmath::Point3;
use cgmath::Vector3 as Vec3;
use cgmath::prelude::*;
use egui_winit::winit::dpi::PhysicalPosition;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);

// Model ------------------------------------------------------------------
pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vec3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl ControllerModel for Camera {
    fn to_matrix(&self) -> cgmath::Matrix4<f32> {
        // Moves world to be at position and rotation of the camera
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        // Projects the camera's viewbox to rectangular frustum
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        // Convert OpenGL coords to WGPU coords
        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            eye: cgmath::Point3::new(0.0, 1.0, 2.0),
            target: cgmath::Point3::new(0.0, 0.0, 0.0),
            up: cgmath::Vector3::new(0.0, 1.0, 0.0),
            aspect: 1.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly, so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }
}

impl ControllerUniform<Camera> for CameraUniform {
    fn update_matrix(&mut self, model: &Camera) {
        self.view_proj = model.to_matrix().into();
    }
}
// -------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct CameraControllerFPS {
    position: Point3<f32>,
    forwards: Vec3<f32>,
    right: Vec3<f32>,
    up: Vec3<f32>,
    yaw: f32,
    pitch: f32,

    walk_speed: f32,
    sensitivity: f32,
}

impl Controller<Camera> for CameraControllerFPS {
    fn update_model(&self, camera: &mut Camera) {
        camera.up = self.up;
        camera.eye = self.position;
        camera.target = self.position + self.forwards;
    }

    fn update_controller(&mut self, camera: &Camera) {
        self.up = camera.up;
        self.forwards = camera.target - camera.eye;
        self.position = camera.eye;
    }
}

impl KeybindHandler<CameraKeybind> for CameraControllerFPS {
    fn handle_keybind(&mut self, keybind: &CameraKeybind, is_pressed: bool) -> anyhow::Result<()> {
        #[allow(unreachable_patterns)]
        match keybind {
            CameraKeybind::Forward => {
                self.position += self.forwards * self.walk_speed;
            }
            CameraKeybind::Left => {
                self.position -= self.right * self.walk_speed;
            }
            CameraKeybind::Backward => {
                self.position -= self.forwards * self.walk_speed;
            }
            CameraKeybind::Right => {
                self.position += self.right * self.walk_speed;
            }
            CameraKeybind::Up => {
                self.position += self.up * self.walk_speed;
            }
            CameraKeybind::Down => {
                self.position -= self.up * self.walk_speed;
            }
            _ => {}
        }

        Ok(())
    }
}

impl MouseHandler for CameraControllerFPS {
    fn update_model_mouse(
        &mut self,
        position: PhysicalPosition<f64>,
        center: PhysicalPosition<f64>,
    ) {
        self.spin(
            ((center.x - position.x) as f32) * self.sensitivity / 50.0,
            (center.y - position.y) as f32,
        );
    }
}

impl CameraControllerFPS {
    pub fn new() -> Self {
        let position = Point3::new(0.0, 0.0, 0.0);
        let forwards = Vec3::new(1.0, 0.0, 0.0);
        let right = Vec3::new(0.0, 0.0, 1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);

        Self {
            position,
            forwards,
            right,
            up,
            yaw: 0.0,
            pitch: 0.0,
            walk_speed: 0.1,
            sensitivity: 50.0,
        }
    }

    pub fn spin(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;
        self.pitch += delta_pitch;

        if self.yaw > 360.0 {
            self.yaw -= 360.0;
        } else if self.yaw < 0.0 {
            self.yaw += 360.0;
        }

        self.pitch = self.pitch.clamp(-89.0, 89.0);

        let (yaw_rad, pitch_rad) = (self.yaw.to_radians(), self.pitch.to_radians());

        let cos_yaw = yaw_rad.cos();
        let sin_yaw = yaw_rad.sin();
        let cos_pitch = pitch_rad.cos();
        let sin_pitch = pitch_rad.sin();

        self.forwards.x = cos_yaw * cos_pitch;
        self.forwards.z = -sin_yaw * cos_pitch;
        self.forwards.y = sin_pitch;
        self.forwards = self.forwards.normalize();

        self.up = Vec3::unit_y();
        self.right = self.forwards.cross(Vec3::unit_y()).normalize();
        self.up = self.right.cross(self.forwards).normalize();
    }

    pub fn walk(&mut self, delta_right: f32, delta_forward: f32) {
        let y = self.position.y;
        self.position += self.right * delta_right;
        self.position += self.forwards * delta_forward;
        self.position.y = y;
    }
}

pub struct CameraControllerEditor {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraControllerEditor {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
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

    fn update_controller(&mut self, model: &Camera) {}
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
