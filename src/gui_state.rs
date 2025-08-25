use crate::gui;
use egui::{Align2, Context};

#[derive(Debug, Default)]
pub struct GuiVersion1 {
    x: f32,
    y: f32,
    z: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    scale_x: f32,
    scale_y: f32,
    scale_z: f32,
}

impl gui::GuiState for GuiVersion1 {
    fn show_gui(&mut self, ui: &Context) {
        egui::Window::new("Streamline CFD")
            // .vscroll(true)
            .default_open(true)
            .max_width(1000.0)
            .max_height(800.0)
            .default_width(800.0)
            .resizable(true)
            .anchor(Align2::LEFT_TOP, [0.0, 0.0])
            .show(ui, |ui| {
                // if ui.add(egui::Button::new("Click me")).clicked() {
                //     println!("PRESSED")
                // }
                ui.label("Slider");
                ui.add(egui::Slider::new(&mut self.x, 0.0..=120.0).text("x"));
                ui.add(egui::Slider::new(&mut self.y, 0.0..=120.0).text("y"));
                ui.add(egui::Slider::new(&mut self.z, 0.0..=120.0).text("z"));
                ui.add(egui::Slider::new(&mut self.rot_x, 0.0..=120.0).text("Pitch"));
                ui.add(egui::Slider::new(&mut self.rot_y, 0.0..=120.0).text("Roll"));
                ui.add(egui::Slider::new(&mut self.rot_z, 0.0..=120.0).text("Yaw"));
                ui.add(egui::Slider::new(&mut self.scale_x, 0.0..=120.0).text("scale x"));
                ui.add(egui::Slider::new(&mut self.scale_y, 0.0..=120.0).text("scale y"));
                ui.add(egui::Slider::new(&mut self.scale_z, 0.0..=120.0).text("scale z"));
                ui.end_row();
            });
    }
}
