use crate::backend::renderer::controller::{ControllerModel, ControllerUniform};
use cgmath::Point3;
use cgmath::Vector3 as Vec3;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);

// model ------------------------------------------------------------------
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
