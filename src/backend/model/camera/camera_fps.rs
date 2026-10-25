use crate::backend::model::camera::camera_model::Camera;
use crate::backend::renderer::controller::{CameraControllerTrait, Controller, MouseHandler};
use crate::keybind::{CameraKeybind, KeybindHandler};
use cgmath::Point3;
use cgmath::Vector3 as Vec3;
use cgmath::prelude::*;

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

impl MouseHandler for CameraControllerFPS {
    fn update_model_mouse(&mut self, delta: &(f64, f64)) {
        self.spin(
            (-delta.0 as f32) * self.sensitivity / 50.0,
            (-delta.1 as f32) * self.sensitivity / 100.0,
        );
    }
}

impl KeybindHandler<CameraKeybind> for CameraControllerFPS {
    fn handle_keybind(&mut self, keybind: &CameraKeybind, _: bool) -> anyhow::Result<()> {
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

impl CameraControllerTrait for CameraControllerFPS {}

impl Default for CameraControllerFPS {
    fn default() -> Self {
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
            sensitivity: 25.0,
        }
    }
}

impl CameraControllerFPS {
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

    // pub fn walk(&mut self, delta_right: f32, delta_forward: f32) {
    //     let y = self.position.y;
    //     self.position += self.right * delta_right;
    //     self.position += self.forwards * delta_forward;
    //     self.position.y = y;
    // }
}
