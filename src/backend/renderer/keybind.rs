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

// Create procedural macros
pub trait KeybindTrait {}
impl KeybindTrait for CameraKeybind {}
impl KeybindTrait for TransformKeybind {}

pub trait KeybindHandler<T: KeybindTrait> {
    fn handle_keybind(&mut self, keybind: T, is_pressed: bool) -> anyhow::Result<()>;
}
