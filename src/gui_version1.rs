use crate::util::FloatWrapper;
use crate::{instance, model, resources, transform};
use cgmath::prelude::*;
use cgmath::{Euler, Quaternion, Rad, Vector3};
use egui::emath::Numeric;
use egui::{Align2, CollapsingHeader, Context, Slider, Window};
use std::f32::consts::PI;
use std::ops::RangeInclusive;
use tobj::Model;
use wgpu::util::DeviceExt;

#[derive(Debug, Clone, Copy)]
pub struct GuiOptions {
    pub drag_value_speed: f64,
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

pub struct GuiState<'a> {
    pub models_instance_list: &'a mut model::ModelInstanceList,
    pub transform_object: &'a mut transform::Transform,
    pub window: &'a winit::window::Window,

    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub texture_bind_group_layout: &'a wgpu::BindGroupLayout,
}

impl GuiState<'_> {
    pub async fn add_instance_by_name(&mut self, name: &str) -> anyhow::Result<()> {
        // Add model instance
        self.models_instance_list
            .add_model_instances(
                name,
                vec![Vector3::new(0.0, 0.0, 0.0)],
                self.device,
                self.queue,
                self.texture_bind_group_layout,
            )
            .await?;
        Ok(())
    }
}

pub fn show_gui<'a>(ctx: &'a Context, opt: &mut GuiOptions, state: &'a mut GuiState<'a>) {
    let mut gui_viewer = GuiViewer::new(ctx, *opt, state);
    gui_viewer.window_global_settings_menu();
    gui_viewer.window_instance_menu();
}

struct GuiViewer<'a> {
    pub ctx: &'a Context,
    pub options: GuiOptions,
    pub state: &'a mut GuiState<'a>,
}

impl<'a> GuiViewer<'a> {
    pub fn new(ctx: &'a Context, options: GuiOptions, state: &'a mut GuiState<'a>) -> Self {
        Self {
            ctx,
            options,
            state,
        }
    }

    fn window_global_settings_menu(&mut self) {
        Window::new("Global Settings")
            // .vscroll(true)
            .default_open(true)
            .max_width(1000.0)
            .max_height(800.0)
            .movable(true)
            .default_width(800.0)
            .resizable(true)
            .anchor(Align2::LEFT_TOP, [0.0, 0.0])
            .show(self.ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Probs bad for performance

                    let rotation = &mut self.state.transform_object.rotation.0;
                    let translation = &mut self.state.transform_object.translation.0;
                    let scale = &mut self.state.transform_object.scale.0;

                    self.options
                        .create_slider(ui, "x", -4.0..=4.0, &mut translation.x);
                    self.options
                        .create_slider(ui, "y", -4.0..=4.0, &mut translation.y);
                    self.options
                        .create_slider(ui, "z", -4.0..=4.0, &mut translation.z);
                    self.options
                        .create_slider(ui, "Pitch", -PI..=PI, rotation.x.get_mut());
                    self.options
                        .create_slider(ui, "Roll", -PI..=PI, rotation.y.get_mut());
                    self.options
                        .create_slider(ui, "Yaw", -PI..=PI, rotation.z.get_mut());
                    self.options
                        .create_slider(ui, "scale x", -4.0..=4.0, scale.x.get_mut());
                    self.options
                        .create_slider(ui, "scale y", -4.0..=4.0, scale.y.get_mut());
                    self.options
                        .create_slider(ui, "scale z", -4.0..=4.0, scale.z.get_mut());
                    ui.end_row();
                });
            });
    }

    fn window_instance_menu(&mut self) {
        let window_size = self.state.window.inner_size();
        Window::new("Instances")
            // .vscroll(true)
            .default_open(true)
            .min_width(800.0)
            .min_height(window_size.height as f32)
            .anchor(Align2::RIGHT_TOP, [0.0, 0.0])
            .show(self.ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.section_instance_creation_menu(ui);
                    self.section_instance_settings_menu(ui);
                    ui.end_row();
                });
            });
    }

    fn section_instance_creation_menu(&mut self, ui: &mut egui::Ui) {
        if ui.add(egui::Button::new("Add cube")).clicked() {
            println!("Loading model");

            // Based as hell
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                if self.state.add_instance_by_name("Cube.obj").await.is_err() {
                    println!("Failed to load model");
                }
                println!(
                    "{:?}",
                    self.state.models_instance_list.get_total_instance_count()
                );
            });
        }

        ui.end_row();
    }

    fn section_instance_settings_menu(&mut self, ui: &mut egui::Ui) {
        CollapsingHeader::new("Instances Settings").show(ui, |ui| {
            for instances in &mut *self.state.models_instance_list.0 {
                for instance in &mut instances.instances {
                    CollapsingHeader::new(instance.name.to_string()).show(ui, |ui| {
                        let position: &mut Vector3<f32> = &mut instance.position;
                        let rotation: &mut Euler<Rad<f32>> = &mut Euler::from(instance.rotation);

                        ui.label("Settings");
                        self.options
                            .create_slider(ui, "x", -200.0..=200.0, &mut position.x);
                        self.options
                            .create_slider(ui, "y", -200.0..=200.0, &mut position.y);
                        self.options
                            .create_slider(ui, "z", -200.0..=200.0, &mut position.z);
                        self.options
                            .create_slider(ui, "Roll", -PI..=PI, &mut rotation.x.0);
                        self.options.create_slider(
                            ui,
                            "Yaw",
                            -PI / 2.0..=PI / 2.0,
                            &mut rotation.y.0,
                        );
                        self.options
                            .create_slider(ui, "Pitch", -PI..=PI, &mut rotation.z.0);
                        ui.end_row();
                        instance.rotation = Quaternion::from(*rotation);
                    });
                }
            }
            ui.end_row();
        });
    }
}
