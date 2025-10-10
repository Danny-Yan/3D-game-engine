use crate::backend::renderer::keybind::{AppKeybind, CameraKeybind, Keybind};
use egui_winit::winit::keyboard::KeyCode;

pub fn handle_key(keycode: KeyCode) -> Option<Keybind> {
    match keycode {
        KeyCode::Escape => Some(Keybind::App(AppKeybind::Exit)),
        KeyCode::Space => Some(Keybind::App(AppKeybind::SwitchRender)),
        KeyCode::KeyY => Some(Keybind::App(AppKeybind::SwitchScene)),
        KeyCode::KeyR => Some(Keybind::App(AppKeybind::ToggleRotationMode)),
        KeyCode::KeyX => Some(Keybind::App(AppKeybind::ToggleCameraMode)),
        KeyCode::KeyW | KeyCode::ArrowUp => Some(Keybind::Camera(CameraKeybind::Forward)),
        KeyCode::KeyS | KeyCode::ArrowDown => Some(Keybind::Camera(CameraKeybind::Backward)),
        KeyCode::KeyA | KeyCode::ArrowLeft => Some(Keybind::Camera(CameraKeybind::Left)),
        KeyCode::KeyD | KeyCode::ArrowRight => Some(Keybind::Camera(CameraKeybind::Right)),
        _ => None,
    }
}
