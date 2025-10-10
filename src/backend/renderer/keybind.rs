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
