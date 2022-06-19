use crate::{
    backgrounds::{GradientBackground, SkyMap, UniformBackground},
    core::camera::Camera,
    utils::color::Color,
    utils::types::Vec3,
};
use egui::{InnerResponse, Ui};

pub trait Editable {
    fn display_ui(&mut self, ui: &mut Ui, _modified: &mut bool) -> InnerResponse<()> {
        ui.group(|_| {})
    }
}

impl Editable for Vec3 {
    fn display_ui(&mut self, ui: &mut Ui, modified: &mut bool) -> InnerResponse<()> {
        ui.horizontal(|ui| {
            *modified |= ui
                .add(
                    egui::widgets::DragValue::new(&mut self.x)
                        .speed(0.02)
                        .prefix("x: "),
                )
                .changed();
            *modified |= ui
                .add(
                    egui::widgets::DragValue::new(&mut self.y)
                        .speed(0.02)
                        .prefix("y: "),
                )
                .changed();
            *modified |= ui
                .add(
                    egui::widgets::DragValue::new(&mut self.z)
                        .speed(0.02)
                        .prefix("z: "),
                )
                .changed();
        })
    }
}

impl Editable for UniformBackground {
    fn display_ui(&mut self, ui: &mut Ui, modified: &mut bool) -> InnerResponse<()> {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Color:");
                let mut rgb = self.color.data();
                *modified |= ui.color_edit_button_rgb(&mut rgb).changed();
                self.color = Color::from(rgb);
            });
        })
    }
}

impl Editable for GradientBackground {
    fn display_ui(&mut self, ui: &mut Ui, modified: &mut bool) -> InnerResponse<()> {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Top:");
                let mut rgb = self.top.data();
                *modified |= ui.color_edit_button_rgb(&mut rgb).changed();
                self.top = Color::from(rgb);

                ui.label("Bottom:");
                let mut rgb = self.bottom.data();
                *modified |= ui.color_edit_button_rgb(&mut rgb).changed();
                self.bottom = Color::from(rgb);
            });
        })
    }
}

impl Editable for SkyMap {}

impl Editable for Camera {
    fn display_ui(&mut self, ui: &mut Ui, modified: &mut bool) -> InnerResponse<()> {
        ui.group(|ui| {
            ui.label("Look from:");
            self.lookfrom.display_ui(ui, modified);
            ui.label("Look at:");
            self.lookat.display_ui(ui, modified);
            ui.label("Vertical:");
            self.vertical.display_ui(ui, modified);
            ui.label("Field of view:");
            *modified |= ui
                .add(egui::Slider::new(&mut self.vertical_fov, 0.60..=120.0))
                .changed();
        })
    }
}
