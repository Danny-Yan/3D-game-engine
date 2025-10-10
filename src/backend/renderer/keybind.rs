use egui_winit::winit::keyboard::KeyCode;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::cmp::PartialEq;
use std::collections::HashMap;

pub enum CameraKeybind {
    Forward,
    Backward,
    Left,
    Right,
}

pub enum TransformKeybind {
    RotateX,
    RotateY,
    RotateZ,
    TranslateX,
    TranslateY,
    TranslateZ,
    ScaleX,
    ScaleY,
    ScaleZ,
}

pub enum AppKeybind {
    ToggleCameraMode,
    ToggleRotationMode,
    SwitchRender,
    SwitchScene,
    Exit,
}

// Create procedural macros
pub trait KeybindTrait {}
impl KeybindTrait for CameraKeybind {}
impl KeybindTrait for TransformKeybind {}
impl KeybindTrait for AppKeybind {}

pub enum Keybind {
    Camera(CameraKeybind),
    Transform(TransformKeybind),
    App(AppKeybind),
}

// Keybind::Camera(CameraKeybind::Forward);

pub trait KeybindHandler<T: KeybindTrait> {
    fn handle_keybind(&mut self, keybind: &T, is_pressed: bool) -> anyhow::Result<()>;
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, FromPrimitive)]
pub enum InputContexts {
    Editor = 0,
    Camera,
}

pub struct InputMapper {
    context: InputContexts,
    context_table: HashMap<InputContexts, HashMap<KeyCode, Keybind>>,
}

impl InputMapper {
    pub fn new() -> Self {
        #[rustfmt::skip]
        let editor_context = HashMap::from([
            (KeyCode::Escape,       Keybind::App(AppKeybind::Exit)),
            (KeyCode::Space,        Keybind::App(AppKeybind::SwitchRender)),
            (KeyCode::KeyY,         Keybind::App(AppKeybind::SwitchScene)),
            (KeyCode::KeyR,         Keybind::App(AppKeybind::ToggleRotationMode)),
            (KeyCode::KeyX,         Keybind::App(AppKeybind::ToggleCameraMode)),
            (KeyCode::KeyW,         Keybind::Camera(CameraKeybind::Forward)),
            (KeyCode::ArrowUp,      Keybind::Camera(CameraKeybind::Forward)),
            (KeyCode::KeyS,         Keybind::Camera(CameraKeybind::Backward)),
            (KeyCode::ArrowDown,    Keybind::Camera(CameraKeybind::Backward)),
            (KeyCode::KeyA,         Keybind::Camera(CameraKeybind::Left)),
            (KeyCode::ArrowLeft,    Keybind::Camera(CameraKeybind::Left)),
            (KeyCode::KeyD,         Keybind::Camera(CameraKeybind::Right)),
            (KeyCode::ArrowRight,   Keybind::Camera(CameraKeybind::Right)),
        ]);

        #[rustfmt::skip]
        let camera_context = HashMap::from([
            (KeyCode::Escape,       Keybind::App(AppKeybind::Exit)),
            (KeyCode::KeyX,         Keybind::App(AppKeybind::ToggleCameraMode)),
        ]);

        let context_table = HashMap::from([
            (InputContexts::Camera, camera_context),
            (InputContexts::Editor, editor_context),
        ]);

        Self {
            context: InputContexts::Editor,
            context_table,
        }
    }

    pub fn switch_contexts(&mut self, context: InputContexts) {
        self.context = context;
    }

    // https://stackoverflow.com/questions/25867875/how-do-i-toggle-through-enum-variants
    pub fn cycle_contexts(&mut self) {
        self.context = match FromPrimitive::from_u8(self.context as u8 + 1) {
            Some(d2) => d2,
            None => FromPrimitive::from_u8(0).unwrap(),
        }
    }

    pub fn handle_key(&mut self, keycode: KeyCode) -> Option<&Keybind> {
        match self.context_table.get(&self.context) {
            Some(context) => match context.get(&keycode) {
                Some(keybind) => Some(keybind),
                _ => None,
            },
            _ => None,
        }
    }
}
