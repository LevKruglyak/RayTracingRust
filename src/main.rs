#![deny(clippy::all)]
#![forbid(unsafe_code)]

use notan::egui::{self, *};
use notan::prelude::*;
use ray_tracing_rust::{render::*, scene::*, sky::*, color, gui::Editable};

#[derive(AppState)]
struct State {
    tex_id: egui::TextureId,
    tex: Texture,
    img_data: Vec<u8>,
    img_size: egui::Vec2,
    demo: RayTracingDemo,
}

static RENDER_WIDTH: i32 = 500;
static RENDER_HEIGHT: i32 = 500;

impl State {
    fn new(gfx: &mut Graphics) -> State {
        let texture = gfx
            .create_texture()
            .with_size(RENDER_WIDTH, RENDER_HEIGHT)
            .with_premultiplied_alpha()
            .build()
            .unwrap();

        let img_size: egui::Vec2 = texture.size().into();
        let tex_id = gfx.egui_register_texture(&texture);

        let mut demo = RayTracingDemo::load(RENDER_WIDTH as u32, RENDER_HEIGHT as u32, "scenes/simple.json");
        let mut img_data = vec![255; (RENDER_WIDTH * RENDER_HEIGHT * 4) as usize];
        demo.render(&mut img_data);

        Self {
            img_size,
            tex: texture,
            img_data,
            tex_id,
            demo,
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::new)
        .add_config(EguiConfig)
        .draw(draw)
        .build()
}

fn draw(gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut output = plugins.egui(|ctx| {
        egui::Window::new("Render Output").show(ctx, |ui| {
            ui.image(state.tex_id, state.img_size);
        });

        egui::SidePanel::left("Settings").show(ctx, |ui| {
                let app = &mut state.demo;
                let mut modified = false;

                ui.label("Samples per pixel:");
                ui.add(egui::Slider::new(
                    &mut app.scene.settings.samples_per_pixel,
                    1..=1000,
                ));
                ui.label("Max ray depth:");
                ui.add(egui::Slider::new(
                    &mut app.scene.settings.max_ray_depth,
                    1..=50,
                ));

                ui.horizontal(|ui| {
                    ui.label("Render mode:");
                    ComboBox::from_label("")
                        .selected_text(format!("{:?}", app.scene.settings.mode))
                        .show_ui(ui, |ui| {
                            if ui
                                .selectable_value(
                                    &mut app.scene.settings.mode,
                                    RenderMode::Full,
                                    "Full",
                                )
                                .clicked()
                            {
                                modified = true
                            };
                            if ui
                                .selectable_value(
                                    &mut app.scene.settings.mode,
                                    RenderMode::Clay,
                                    "Clay",
                                )
                                .clicked()
                            {
                                modified = true
                            };
                            if ui
                                .selectable_value(
                                    &mut app.scene.settings.mode,
                                    RenderMode::Normal,
                                    "Normal",
                                )
                                .clicked()
                            {
                                modified = true
                            };
                            if ui
                                .selectable_value(
                                    &mut app.scene.settings.mode,
                                    RenderMode::Random,
                                    "Random",
                                )
                                .clicked()
                            {
                                modified = true
                            };
                        });
                });

                ui.add(egui::Checkbox::new(
                    &mut app.scene.settings.enable_multithreading,
                    "Enable multithreading",
                ));
                ui.add(egui::Checkbox::new(
                    &mut app.continuous_mode,
                    "Continuous mode",
                ));

                ui.separator();
                ui.heading("Scene Settings");
                ui.collapsing("Camera", |ui| {
                    app.scene.camera.display_ui(ui, &mut modified);
                });

                ui.collapsing("Background", |ui| {
                    app.scene.background.display_ui(ui, &mut modified);
                    ui.horizontal(|ui| {
                        ui.menu_button("Change background", |ui| {
                            if ui.button("Uniform background").clicked() {
                                app.scene.background =
                                    Box::new(UniformBackground::new(color::Color::new(0.8, 0.8, 0.8)));
                                ui.close_menu();
                                modified = true;
                            }
                            if ui.button("Gradient background").clicked() {
                                app.scene.background = Box::new(GradientBackground::new(
                                    color::Color::new(0.5, 0.7, 1.0),
                                    color::Color::new(1.0, 1.0, 1.0),
                                ));
                                ui.close_menu();
                                modified = true;
                            }
                            if ui.button("Sky map").clicked() {}
                        });
                    })
                });

                ui.separator();
                if ui.button("Render Image").clicked() {
                    app.render(&mut state.img_data);
                }

                ui.label(format!("Last render took: {:?}", app.last_time));
                ui.label(format!("Using {:?} threads", rayon::current_num_threads()));

                if modified && app.continuous_mode {
                    app.render(&mut state.img_data);
                }
            });
    });

    output.clear_color(Color::BLACK);

    if output.needs_repaint() {
        gfx.render(&output);
        gfx.update_texture(&mut state.tex)
            .with_data(&state.img_data[..])
            .update()
            .unwrap();
    }
}
