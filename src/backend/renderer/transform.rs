use crate::backend::renderer::controller::{ControllerModel, ControllerUniform};
use crate::util::{RadWrapper, ScaleValue, Vector3Wrapper};
use cgmath::Matrix4;
use std::clone::Clone;

#[derive(Default, Copy, Clone)]
pub struct Transform {
    pub rotation: Vector3Wrapper<RadWrapper>,
    pub translation: Vector3Wrapper<f32>,
    pub scale: Vector3Wrapper<ScaleValue>,
}

impl ControllerModel for Transform {
    fn to_matrix(&self) -> cgmath::Matrix4<f32> {
        let rotation = &self.rotation.0;
        let translation = &self.translation.0;
        let scale = &self.scale.0;

        // Rotation
        let rotation_matrix: Matrix4<f32> = cgmath::Matrix4::from_angle_x(rotation.x.0)
            * cgmath::Matrix4::from_angle_y(rotation.y.0)
            * cgmath::Matrix4::from_angle_z(rotation.z.0);
        // Translation
        let translation_matrix: Matrix4<f32> = cgmath::Matrix4::from_translation(*translation);
        // Scale
        let scale_matrix: Matrix4<f32> =
            cgmath::Matrix4::from_nonuniform_scale(scale.x.0, scale.y.0, scale.z.0);

        rotation_matrix * translation_matrix * scale_matrix
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformUniform {
    matrix: [[f32; 4]; 4],
}
impl TransformUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            matrix: cgmath::Matrix4::identity().into(),
        }
    }
}
impl ControllerUniform<Transform> for TransformUniform {
    fn update_matrix(&mut self, transform: &Transform) {
        self.matrix = transform.to_matrix().into();
    }
}
