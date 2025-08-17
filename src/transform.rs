use crate::keybind::KeybindHandler;
use crate::keybind::TransformKeybind;
use crate::util::{RadWrapper, ScaleValue, TranslationValue, Vector3Wrapper};
use anyhow::Ok;
use cgmath::Matrix4;

#[derive(Default)]
pub struct Transform {
    rotation: Vector3Wrapper<RadWrapper<f32>>,
    translation: Vector3Wrapper<f32>, // WHY DOES THIS NOT WORK
    scale: Vector3Wrapper<ScaleValue>,
}

impl Transform {
    pub fn to_matrix(&self) -> cgmath::Matrix4<f32> {
        // Rotation
        let rotation_matrix: Matrix4<f32> = cgmath::Matrix4::from_angle_x(self.rotation.x().0)
            * cgmath::Matrix4::from_angle_y(self.rotation.y().0)
            * cgmath::Matrix4::from_angle_z(self.rotation.z().0);
        // Translation
        let translation_matrix: Matrix4<f32> =
            cgmath::Matrix4::from_translation(self.translation.0);
        // Scale
        let scale_matrix: Matrix4<f32> = cgmath::Matrix4::from_nonuniform_scale(
            self.scale.x().0,
            self.scale.y().0,
            self.scale.z().0,
        );

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
    rotation_speed: RadWrapper<f32>,
    translation_speed: TranslationValue,
    scale_speed: ScaleValue,

    rotation_truth_table: TrueAxisTable,
    translation_truth_table: TrueAxisTable,
    scale_truth_table: TrueAxisTable,
}

impl TransformController {
    pub fn new() -> Self {
        Self {
            rotation_speed: RadWrapper::<f32>::new(0.2),
            translation_speed: TranslationValue(0.2),
            scale_speed: ScaleValue(1.2),
            ..Default::default()
        }
    }

    fn match_axis<T: Default + Copy>(&self, axis: bool, value: &T) -> T {
        match axis {
            true => *value,
            false => T::default(),
        }
    }

    pub fn update_transform(&mut self, transform: &mut Transform) {
        transform.rotation.0.x = self.match_axis(self.rotation_truth_table.x, &self.rotation_speed);
        transform.rotation.0.y = self.match_axis(self.rotation_truth_table.y, &self.rotation_speed);
        transform.rotation.0.z = self.match_axis(self.rotation_truth_table.z, &self.rotation_speed);
        transform.translation.0.x =
            self.match_axis(self.translation_truth_table.x, &self.translation_speed.0);
        transform.translation.0.y =
            self.match_axis(self.translation_truth_table.y, &self.translation_speed.0);
        transform.translation.0.z =
            self.match_axis(self.translation_truth_table.z, &self.translation_speed.0);
        transform.scale.0.x = self.match_axis(self.scale_truth_table.x, &self.scale_speed);
        transform.scale.0.y = self.match_axis(self.scale_truth_table.y, &self.scale_speed);
        transform.scale.0.z = self.match_axis(self.scale_truth_table.z, &self.scale_speed);
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
