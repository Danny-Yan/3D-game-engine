use crate::instance;
use crate::util;
use crate::util::FloatWrapper;
use egui::{Align2, Context};
use std::f32::consts::PI;

#[derive(Default)]
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
    pub fn show_gui(&mut self, ctx: &Context, instances: &mut Vec<instance::Instance>) {
        egui::Window::new("Instances")
            // .vscroll(true)
            .default_open(true)
            .max_width(1000.0)
            .max_height(800.0)
            .movable(true)
            .default_width(800.0)
            .resizable(true)
            .anchor(Align2::RIGHT_TOP, [0.0, 0.0])
            .show(ctx, |ui| {
                egui::CollapsingHeader::new("Instances").show(ui, |ui| {
                    for instance in instances {
                        egui::CollapsingHeader::new(instance.name.to_string()).show(ui, |ui| {
                            let position: &mut cgmath::Vector3<f32> = &mut instance.position;
                            let rotation: &mut cgmath::Euler<cgmath::Rad<f32>> =
                                &mut cgmath::Euler::from(instance.rotation);

                            ui.label("Settings");
                            ui.add(egui::Slider::new(&mut position.x, -200.0..=200.0).text("x"));
                            ui.add(egui::Slider::new(&mut position.y, -200.0..=200.0).text("y"));
                            ui.add(egui::Slider::new(&mut position.z, -200.0..=200.0).text("z"));
                            ui.add(egui::Slider::new(&mut rotation.x.0, -PI..=PI).text("Pitch"));
                            ui.add(
                                egui::Slider::new(&mut rotation.y.0, -PI / 2.0..=PI / 2.0)
                                    .text("Roll"),
                            );
                            ui.add(egui::Slider::new(&mut rotation.z.0, -PI..=PI).text("Yaw"));
                            // ui.add(
                            //     egui::Slider::new(self.scale_x.get_mut(), -4.0..=4.0).text("scale x"),
                            // );
                            // ui.add(
                            //     egui::Slider::new(self.scale_y.get_mut(), -4.0..=4.0).text("scale y"),
                            // );
                            // ui.add(
                            //     egui::Slider::new(self.scale_z.get_mut(), -4.0..=4.0).text("scale z"),
                            // );
                            ui.end_row();
                            instance.rotation = cgmath::Quaternion::from(*rotation);
                        });
                    }
                    ui.end_row();
                });
            });

        egui::Window::new("Global Settings")
            // .vscroll(true)
            .default_open(true)
            .max_width(1000.0)
            .max_height(800.0)
            .movable(true)
            .default_width(800.0)
            .resizable(true)
            .anchor(Align2::LEFT_TOP, [0.0, 0.0])
            .show(ctx, |ui| {
                // if ui.add(egui::Button::new("Click me")).clicked() {
                //     println!("PRESSED")
                // }
                ui.add(egui::Slider::new(&mut self.x, -4.0..=4.0).text("x"));
                ui.add(egui::Slider::new(&mut self.y, -4.0..=4.0).text("y"));
                ui.add(egui::Slider::new(&mut self.z, -4.0..=4.0).text("z"));
                ui.add(egui::Slider::new(self.rot_x.get_mut(), -4.0..=4.0).text("Pitch"));
                ui.add(egui::Slider::new(self.rot_y.get_mut(), -4.0..=4.0).text("Roll"));
                ui.add(egui::Slider::new(self.rot_z.get_mut(), -4.0..=4.0).text("Yaw"));
                ui.add(egui::Slider::new(self.scale_x.get_mut(), -4.0..=4.0).text("scale x"));
                ui.add(egui::Slider::new(self.scale_y.get_mut(), -4.0..=4.0).text("scale y"));
                ui.add(egui::Slider::new(self.scale_z.get_mut(), -4.0..=4.0).text("scale z"));
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
