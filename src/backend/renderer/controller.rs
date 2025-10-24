use crate::backend::renderer::{
    camera::Camera,
    keybind::{CameraKeybind, KeybindHandler, KeybindTrait},
};
use egui_winit::winit::dpi::PhysicalPosition;

// Model itself
pub trait ControllerModel {
    fn to_matrix(&self) -> cgmath::Matrix4<f32>;
}

// Changes model
pub trait Controller<T: ControllerModel> {
    fn update_model(&self, model: &mut T);

    // optional
    fn update_controller(&mut self, model: &T) {}
}

// Transfer model to uniform buffer
pub trait ControllerUniform<T: ControllerModel> {
    fn update_matrix(&mut self, model: &T);
}

pub trait MouseHandler {
    fn update_model_mouse(
        &mut self,
        position: PhysicalPosition<f64>,
        center: PhysicalPosition<f64>,
    ) {
    }
}

// General controller trait (model + uniform + controller)
pub trait GeneralControllerTrait<T, U>: KeybindHandler<T> + Controller<U>
where
    T: KeybindTrait,
    U: ControllerModel,
{
}
impl<W, T, U> GeneralControllerTrait<T, U> for W
where
    T: KeybindTrait,
    U: ControllerModel,
    W: KeybindHandler<T> + Controller<U>,
{
}

pub trait CameraControllerTrait:
    GeneralControllerTrait<CameraKeybind, Camera> + MouseHandler
{
}

impl<W> CameraControllerTrait for W where
    W: GeneralControllerTrait<CameraKeybind, Camera> + MouseHandler
{
}

// General controller + keybind trait struct
pub struct CombinedControllerStruct<T: ControllerModel, U: KeybindTrait> {
    pub model: T,
    pub uniform: Box<dyn ControllerUniform<T>>,
    pub controller: Vec<Box<dyn GeneralControllerTrait<U, T>>>,
}
