use crate::keybind::KeybindHandler;
use crate::keybind::TransformKeybind;
use anyhow::Ok;
use cgmath::Matrix4;

pub struct Transform {
    rotation: cgmath::Vector3<cgmath::Rad<f32>>,
    translation: cgmath::Vector3<f32>,
    scale: cgmath::Vector3<f32>,
}

impl Transform {
    pub fn to_matrix(&self) -> cgmath::Matrix4<f32> {
        // Rotation
        let rotation_matrix: Matrix4<f32> = cgmath::Matrix4::from_angle_x(self.rotation.x)
            * cgmath::Matrix4::from_angle_y(self.rotation.y)
            * cgmath::Matrix4::from_angle_z(self.rotation.z);
        // Translation
        let translation_matrix: Matrix4<f32> = cgmath::Matrix4::from_translation(self.translation);
        // Scale
        let scale_matrix: Matrix4<f32> =
            cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

        translation_matrix * rotation_matrix * scale_matrix
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            rotation: cgmath::Vector3::new(cgmath::Rad(0.0), cgmath::Rad(0.0), cgmath::Rad(0.0)),
            translation: cgmath::Zero::zero(),
            scale: cgmath::Vector3::new(1.0, 1.0, 1.0),
        }
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

    pub fn update_matrix(&mut self, transform: &Transform) {
        self.matrix = transform.to_matrix().into();
    }
}

#[derive(Default)]
struct TrueAxisTable {
    x: bool,
    y: bool,
    z: bool,
}

#[derive(Default)]
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
            rotation_speed: 0.2,
            translation_speed: 0.2,
            scale_speed: 0.2,
            ..Default::default()
        }
    }

    pub fn update_transform(&mut self, transform: &Transform) {
        todo!();
    }
}

impl KeybindHandler<TransformKeybind> for TransformController {
    fn handle_keybind(
        &mut self,
        keycode: TransformKeybind,
        is_pressed: bool,
    ) -> anyhow::Result<()> {
        // // TAHNK YOU KEVIN
        // let balls = keycode
        //     .cast_to()
        //     .downcast_ref::<CameraKeybind>()
        //     .ok_or(wgpu::wgc::validation::InputError::Missing)?;
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
