use cgmath::Vector3 as Vec3;
use cgmath::prelude::*;

struct Camera {
    position: Vec3<f32>,
    forwards: Vec3<f32>,
    right: Vec3<f32>,
    up: Vec3<f32>,
    yaw: f32,
    pitch: f32,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new() -> Self {
        let position = Vec3::new(-5.0, 0.0, 2.0);
        let forwards = Vec3::new(1.0, 0.0, 0.0);
        let right = Vec3::new(0.0, -1.0, 0.0);
        let up = Vec3::new(0.0, 0.0, 1.0);

        let yaw = -90.0;
        let pitch = 0.0;

        let aspect = 1.0;
        let fovy = 45.0;
        let znear = 0.1;
        let zfar = 100.0;

        Self {
            position,
            forwards,
            right,
            up,
            yaw,
            pitch,
            aspect,
            fovy,
            znear,
            zfar,
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
        self.forwards.y = sin_yaw * cos_pitch;
        self.forwards.z = sin_pitch;

        // self.forwards = self.forwards.normalize();

        self.up = Vec3::unit_z();

        self.right = self.forwards.cross(Vec3::unit_z()).normalize();
        self.up = self.right.cross(self.forwards).normalize();
    }

    pub fn walk(&mut self, delta_right: f32, delta_forward: f32) {
        let z = self.position.z;
        self.position += self.right * delta_right;
        self.position += self.forwards * delta_forward;
        self.position.z = z;
    }
}
