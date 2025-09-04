use crate as wgpu_engine;
use crate::util::{FloatWrapper, RadWrapper, ScaleValue};
use cgmath::{Euler, Quaternion, Rad, Vector3};
use egui::emath::Numeric;
use egui::{Align2, CollapsingHeader, Context, Slider, Window};
use std::f32::consts::PI;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy)]
pub struct GuiOptions {
    drag_value_speed: f64,
}
impl Default for GuiOptions {
    fn default() -> Self {
        Self {
            drag_value_speed: 0.1,
        }
    }
}

impl GuiOptions {
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
}

pub fn show_gui(ctx: &Context, opt: &mut GuiOptions, state: &mut wgpu_engine::State) {
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
                for instance in &mut state.instances {
                    CollapsingHeader::new(instance.name.to_string()).show(ui, |ui| {
                        let position: &mut Vector3<f32> = &mut instance.position;
                        let rotation: &mut Euler<Rad<f32>> = &mut Euler::from(instance.rotation);

                        ui.label("Settings");
                        opt.create_slider(ui, "x", -200.0..=200.0, &mut position.x);
                        opt.create_slider(ui, "y", -200.0..=200.0, &mut position.y);
                        opt.create_slider(ui, "z", -200.0..=200.0, &mut position.z);
                        opt.create_slider(ui, "Roll", -PI..=PI, &mut rotation.x.0);
                        opt.create_slider(ui, "Yaw", -PI / 2.0..=PI / 2.0, &mut rotation.y.0);
                        opt.create_slider(ui, "Pitch", -PI..=PI, &mut rotation.z.0);
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
            let rotation = &mut state.transform_object.rotation.0;
            let translation = &mut state.transform_object.translation.0;
            let scale = &mut state.transform_object.scale.0;
            opt.create_slider(ui, "x", -4.0..=4.0, &mut translation.x);
            opt.create_slider(ui, "y", -4.0..=4.0, &mut translation.y);
            opt.create_slider(ui, "z", -4.0..=4.0, &mut translation.z);
            opt.create_slider(ui, "Pitch", -PI..=PI, &mut rotation.x.get_mut());
            opt.create_slider(ui, "Roll", -PI..=PI, &mut rotation.y.get_mut());
            opt.create_slider(ui, "Yaw", -PI..=PI, &mut rotation.z.get_mut());
            opt.create_slider(ui, "scale x", -4.0..=4.0, &mut scale.x.get_mut());
            opt.create_slider(ui, "scale y", -4.0..=4.0, &mut scale.y.get_mut());
            opt.create_slider(ui, "scale z", -4.0..=4.0, &mut scale.z.get_mut());
            ui.end_row();
        });
}
