use crate::instance;
use crate::util::{FloatWrapper, RadWrapper, ScaleValue};
use cgmath::{Euler, Quaternion, Rad, Vector3};
use egui::emath::Numeric;
use egui::{Align2, CollapsingHeader, Context, Slider, Window};
use std::f32::consts::PI;
use std::ops::RangeInclusive;

#[derive(Default, Debug, Copy, Clone)]
pub struct GuiVersion1 {
    x: f32,
    y: f32,
    z: f32,
    rot_x: RadWrapper,
    rot_y: RadWrapper,
    rot_z: RadWrapper,
    scale_x: ScaleValue,
    scale_y: ScaleValue,
    scale_z: ScaleValue,
    drag_value_speed: f64,
}

impl GuiVersion1 {
    pub fn new() -> Self {
        Self {
            drag_value_speed: 0.001, // Precision
            ..Default::default()
        }
    }
    pub fn show_gui(&mut self, ctx: &Context, instances: &mut Vec<instance::Instance>) {
        Window::new("Instances")
            // .vscroll(true)
            .default_open(true)
            .max_width(1000.0)
            .max_height(800.0)
            .movable(true)
            .default_width(800.0)
            .resizable(true)
            .anchor(Align2::RIGHT_TOP, [0.0, 0.0])
            .show(ctx, |ui| {
                CollapsingHeader::new("Instances").show(ui, |ui| {
                    for instance in instances {
                        CollapsingHeader::new(instance.name.to_string()).show(ui, |ui| {
                            let position: &mut Vector3<f32> = &mut instance.position;
                            let rotation: &mut Euler<Rad<f32>> =
                                &mut Euler::from(instance.rotation);

                            ui.label("Settings");
                            self.create_slider(ui, "x", -200.0..=200.0, &mut position.x);
                            self.create_slider(ui, "y", -200.0..=200.0, &mut position.y);
                            self.create_slider(ui, "z", -200.0..=200.0, &mut position.z);
                            self.create_slider(ui, "Roll", -PI..=PI, &mut rotation.x.0);
                            self.create_slider(ui, "Yaw", -PI / 2.0..=PI / 2.0, &mut rotation.y.0);
                            self.create_slider(ui, "Pitch", -PI..=PI, &mut rotation.z.0);
                            ui.end_row();
                            instance.rotation = Quaternion::from(*rotation);
                        });
                    }
                    ui.end_row();
                });
            });

        Window::new("Global Settings")
            // .vscroll(true)
            .default_open(true)
            .max_width(1000.0)
            .max_height(800.0)
            .movable(true)
            .default_width(800.0)
            .resizable(true)
            .anchor(Align2::LEFT_TOP, [0.0, 0.0])
            .show(ctx, |ui| {
                self.create_slider(ui, "x", -4.0..=4.0, &mut self.x);
                self.create_slider(ui, "y", -4.0..=4.0, &mut self.y);
                self.create_slider(ui, "z", -4.0..=4.0, &mut self.z);
                self.create_slider(ui, "Pitch", -4.0..=4.0, self.rot_x.get_mut());
                self.create_slider(ui, "Roll", -4.0..=4.0, self.rot_y.get_mut());
                self.create_slider(ui, "Yaw", -4.0..=4.0, self.rot_z.get_mut());
                self.create_slider(ui, "scale x", -4.0..=4.0, self.scale_x.get_mut());
                self.create_slider(ui, "scale y", -4.0..=4.0, self.scale_y.get_mut());
                self.create_slider(ui, "scale z", -4.0..=4.0, self.scale_z.get_mut());
                ui.end_row();
            });
    }

    fn create_slider<Num: Numeric>(
        self,
        ui: &mut egui::Ui,
        label: &str,
        range: RangeInclusive<Num>,
        value: &mut Num,
    ) {
        ui.add(
            Slider::new(value, range)
                .text(label)
                .drag_value_speed(self.drag_value_speed),
        );
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

    pub fn get_rot_x(&self) -> RadWrapper {
        self.rot_x
    }

    pub fn get_rot_y(&self) -> RadWrapper {
        self.rot_y
    }

    pub fn get_rot_z(&self) -> RadWrapper {
        self.rot_z
    }

    pub fn get_scale_x(&self) -> ScaleValue {
        self.scale_x
    }

    pub fn get_scale_y(&self) -> ScaleValue {
        self.scale_y
    }

    pub fn get_scale_z(&self) -> ScaleValue {
        self.scale_z
    }
}
