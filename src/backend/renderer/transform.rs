use crate::backend::renderer::controller::{Controller, ControllerModel, ControllerUniform};
use crate::keybind::KeybindHandler;
use crate::keybind::TransformKeybind;
use crate::util::{RadWrapper, ScaleValue, Vector3Wrapper};
use anyhow::Ok;
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

#[derive(Default, Copy, Clone)]
struct TrueAxisTable {
    x: bool,
    y: bool,
    z: bool,
}

#[derive(Default, Copy, Clone)]
pub struct TransformController {
    rotation_speed: f32,
    translation_speed: f32,
    scale_speed: f32,

    rotation_truth_table: TrueAxisTable,
    translation_truth_table: TrueAxisTable,
    scale_truth_table: TrueAxisTable,
}

impl TransformController {
    pub fn new() -> Self {
        Self {
            rotation_speed: 0.1,
            translation_speed: 0.1,
            scale_speed: 1.1,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    fn match_axis(&self, axis: bool, value: &f32) -> f32 {
        match axis {
            true => *value,
            false => 0.0,
        }
    }

    #[allow(dead_code)]
    fn match_axis_new_default(&self, axis: bool, value: &f32) -> f32 {
        match axis {
            true => *value,
            false => 0.0,
        }
    }
}

// impl Controller<Transform> for TransformController {
//     fn update_model(&self, transform: &mut Transform) {
//         let translation = &mut transform.translation.0;
//         let rotation = &mut transform.rotation.0;
//         let scale = &mut transform.scale.0;

//         // Compress down somehow
//         rotation.x.0.0 += self.match_axis(self.rotation_truth_table.x, &self.rotation_speed);
//         rotation.y.0.0 += self.match_axis(self.rotation_truth_table.y, &self.rotation_speed);
//         rotation.z.0.0 += self.match_axis(self.rotation_truth_table.z, &self.rotation_speed);
//         translation.x += self.match_axis(self.translation_truth_table.x, &self.translation_speed);
//         translation.y += self.match_axis(self.translation_truth_table.y, &self.translation_speed);
//         translation.z += self.match_axis(self.translation_truth_table.z, &self.translation_speed);
//         scale.x.0 += self.match_axis(self.scale_truth_table.x, &self.scale_speed);
//         scale.y.0 += self.match_axis(self.scale_truth_table.y, &self.scale_speed);
//         scale.z.0 += self.match_axis(self.scale_truth_table.z, &self.scale_speed);
//     }
// }

impl KeybindHandler<TransformKeybind> for TransformController {
    fn handle_keybind(
        &mut self,
        keycode: &TransformKeybind,
        is_pressed: bool,
    ) -> anyhow::Result<()> {
        // // TAHNK YOU KEVIN
        // let balls = keycode
        //     .cast_to()
        //     .downcast_ref::<CameraKeybind>()
        //     .ok_or(wgpu::wgc::validation::InputError::Missing)?;

        // Compress down somehow
        match keycode {
            TransformKeybind::RotateX => {
                self.rotation_truth_table.x = is_pressed;
                Ok(())
            }
            TransformKeybind::RotateY => {
                self.rotation_truth_table.y = is_pressed;
                Ok(())
            }
            TransformKeybind::RotateZ => {
                self.rotation_truth_table.z = is_pressed;
                Ok(())
            }
            TransformKeybind::TranslateX => {
                self.translation_truth_table.x = is_pressed;
                Ok(())
            }
            TransformKeybind::TranslateY => {
                self.translation_truth_table.y = is_pressed;
                Ok(())
            }
            TransformKeybind::TranslateZ => {
                self.translation_truth_table.z = is_pressed;
                Ok(())
            }
            TransformKeybind::ScaleX => {
                self.scale_truth_table.x = is_pressed;
                Ok(())
            }
            TransformKeybind::ScaleY => {
                self.scale_truth_table.y = is_pressed;
                Ok(())
            }
            TransformKeybind::ScaleZ => {
                self.scale_truth_table.z = is_pressed;
                Ok(())
            }
        }
    }
}
