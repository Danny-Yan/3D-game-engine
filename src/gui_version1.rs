use crate::util;
use egui::{Align2, Context};

#[derive(Debug, Default)]
pub struct GuiVersion1 {
    x: f32,
    y: f32,
    z: f32,
    rot_x: util::RadWrapper,
    rot_y: util::RadWrapper,
    rot_z: util::RadWrapper,
    scale_x: util::ScaleValue,
    scale_y: util::ScaleValue,
    scale_z: util::ScaleValue,
}

impl GuiVersion1 {
    pub fn show_gui(&mut self, ui: &Context) {
        egui::Window::new("Settings")
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
                ui.add(egui::Slider::new(&mut self.x, -4.0..=4.0).text("x"));
                ui.add(egui::Slider::new(&mut self.y, -4.0..=4.0).text("y"));
                ui.add(egui::Slider::new(&mut self.z, -4.0..=4.0).text("z"));
                ui.add(egui::Slider::new(&mut self.rot_x.0.0, -4.0..=4.0).text("Pitch"));
                ui.add(egui::Slider::new(&mut self.rot_y.0.0, -4.0..=4.0).text("Roll"));
                ui.add(egui::Slider::new(&mut self.rot_z.0.0, -4.0..=4.0).text("Yaw"));
                ui.add(egui::Slider::new(&mut self.scale_x.0, -4.0..=4.0).text("scale x"));
                ui.add(egui::Slider::new(&mut self.scale_y.0, -4.0..=4.0).text("scale y"));
                ui.add(egui::Slider::new(&mut self.scale_z.0, -4.0..=4.0).text("scale z"));
                ui.end_row();
            });
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_z(&self) -> f32 {
        self.z
    }

    pub fn get_rot_x(&self) -> util::RadWrapper {
        self.rot_x
    }

    pub fn get_rot_y(&self) -> util::RadWrapper {
        self.rot_y
    }

    pub fn get_rot_z(&self) -> util::RadWrapper {
        self.rot_z
    }

    pub fn get_scale_x(&self) -> util::ScaleValue {
        self.scale_x
    }

    pub fn get_scale_y(&self) -> util::ScaleValue {
        self.scale_y
    }

    pub fn get_scale_z(&self) -> util::ScaleValue {
        self.scale_z
    }
}
