use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use egui::{ClippedMesh, ComboBox, Context, TexturesDelta};
use egui_wgpu_backend::{BackendError, RenderPass, ScreenDescriptor};
use pixels::{wgpu, PixelsContext};
use ray_tracing_rust::backgrounds::{GradientBackground, SkyMap, UniformBackground};
use ray_tracing_rust::core::mesh::Mesh;
use ray_tracing_rust::core::render::{render, RenderTarget};
use ray_tracing_rust::core::scene::RenderMode;
use ray_tracing_rust::core::scene::Scene;
use ray_tracing_rust::gui::gui::Editable;
use ray_tracing_rust::materials::{Dielectric, Emission, Lambertian, Metal, MixMaterial};
use ray_tracing_rust::objects::Sphere;
use ray_tracing_rust::utils::color::Color;
use ray_tracing_rust::utils::types::*;
use ray_tracing_rust::volume::*;
use winit::window::Window;

/// Manages all state required for rendering egui over `Pixels`.
pub(crate) struct Framework {
    // State for egui.
    egui_ctx: Context,
    egui_state: egui_winit::State,
    screen_descriptor: ScreenDescriptor,
    rpass: RenderPass,
    paint_jobs: Vec<ClippedMesh>,
    textures: TexturesDelta,

    // State for the GUI
    gui: Gui,
}

impl Framework {
    /// Create egui.
    pub(crate) fn new(
        width: u32,
        height: u32,
        scale_factor: f32,
        pixels: &pixels::Pixels,
        target: Rc<RefCell<RenderTarget>>,
    ) -> Self {
        let max_texture_size = pixels.device().limits().max_texture_dimension_2d as usize;

        let egui_ctx = Context::default();
        let egui_state = egui_winit::State::from_pixels_per_point(max_texture_size, scale_factor);
        let screen_descriptor = ScreenDescriptor {
            physical_width: width,
            physical_height: height,
            scale_factor,
        };
        let rpass = RenderPass::new(pixels.device(), pixels.render_texture_format(), 1);
        let textures = TexturesDelta::default();

        fn setup_scene() -> Scene {
            let mut scene = Scene::default();
            scene.camera.lookfrom.x = -3.0;
            scene.camera.lookfrom.y = 2.0;
            scene.camera.lookfrom.z = 4.0;

            scene.camera.lookat.x = 0.0;
            scene.camera.lookat.y = 2.0;
            scene.camera.lookat.z = -1.0;

            scene.settings.max_ray_depth = 3;
            scene.background = Box::new(SkyMap::new("assets/lake.exr"));

            let lambertian_material =
                scene.add_material(Box::new(Lambertian::new(Color::new(0.9, 0.2, 0.2))));
            let metal_material =
                scene.add_material(Box::new(Metal::new(Color::new(0.8, 0.2, 0.2), 0.02)));
            let glass_material = scene.add_material(Box::new(Dielectric::new(1.5)));
            let ground_material = scene.add_material(Box::new(MixMaterial::new(
                Box::new(Metal::new(Color::new(0.1, 0.1, 0.1), 0.0)),
                Box::new(Lambertian::new(Color::new(0.4, 0.4, 0.4))),
                0.5,
            )));
            let emission_material =
                scene.add_material(Box::new(Emission::new(Color::new(1.0, 1.0, 1.0), 50.0)));
            let isotropic_material =
                scene.add_material(Box::new(Isotropic::new(Color::new(1.0, 1.0, 1.0))));

            let mut mesh = Box::new(Mesh::from_file("assets/house.obj", lambertian_material));
            mesh.build_bvh();
            scene.add_object(mesh);

            let mut mesh = Box::new(Mesh::from_file("assets/plane.obj", ground_material));
            mesh.build_bvh();
            scene.add_object(mesh);

            // scene.add_object(Box::new(Sphere::new(
            //     Vec3::new(0.0, 4.0, -4.0),
            //     1.0,
            //     emission_material,
            // )));

            // let sphere = Box::new(Sphere::new(
            //     Vec3::new(0.0, 0.0, 0.0),
            //     100.0,
            //     isotropic_material,
            // ));
            // scene.add_object(Box::new(Volume::new(sphere, 0.1)));

            // Add a bunch of objects
            // for x in -2..2 {
            //     for y in -5..5 {
            //         for z in -5..5 {
            //             scene.add_object(Box::new(Sphere::new(
            //                 Vec3::new(x as Float, y as Float, z as Float),
            //                 0.5,
            //                 default_material,
            //             )));
            //         }
            //     }
            // }

            scene
        }

        let gui = Gui {
            render_target: target,
            continuous_mode: false,
            last_time: Duration::new(0, 0),
            scene: setup_scene(),
        };

        Self {
            egui_ctx,
            egui_state,
            screen_descriptor,
            rpass,
            paint_jobs: Vec::new(),
            textures,
            gui,
        }
    }

    /// Handle input events from the window manager.
    pub(crate) fn handle_event(&mut self, event: &winit::event::WindowEvent) {
        self.egui_state.on_event(&self.egui_ctx, event);
    }

    /// Resize egui.
    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.screen_descriptor.physical_width = width;
            self.screen_descriptor.physical_height = height;
        }
    }

    /// Update scaling factor.
    pub(crate) fn scale_factor(&mut self, scale_factor: f64) {
        self.screen_descriptor.scale_factor = scale_factor as f32;
    }

    /// Prepare egui.
    pub(crate) fn prepare(&mut self, window: &Window) {
        // Run the egui frame and create all paint jobs to prepare for rendering.
        let raw_input = self.egui_state.take_egui_input(window);
        let output = self.egui_ctx.run(raw_input, |egui_ctx| {
            // Draw the application.
            self.gui.ui(egui_ctx);
        });

        self.textures.append(output.textures_delta);
        self.egui_state
            .handle_platform_output(window, &self.egui_ctx, output.platform_output);
        self.paint_jobs = self.egui_ctx.tessellate(output.shapes);
    }

    /// Render egui.
    pub(crate) fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
    ) -> Result<(), BackendError> {
        // Upload all resources to the GPU.
        self.rpass
            .add_textures(&context.device, &context.queue, &self.textures)?;
        self.rpass.update_buffers(
            &context.device,
            &context.queue,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        // Record all render passes.
        self.rpass.execute(
            encoder,
            render_target,
            &self.paint_jobs,
            &self.screen_descriptor,
            None,
        )?;

        // Cleanup
        let textures = std::mem::take(&mut self.textures);
        self.rpass.remove_textures(textures)
    }
}

/// Application state
struct Gui {
    render_target: Rc<RefCell<RenderTarget>>,
    scene: Scene,
    last_time: Duration,
    continuous_mode: bool,
}

impl Gui {
    /// Create the UI using egui.
    fn ui(&mut self, ctx: &Context) {
        egui::Window::new("Settings")
            .open(&mut true)
            .show(ctx, |ui| {
                let scene = &mut self.scene;
                let mut modified = false;

                ui.label("Samples per pixel:");
                ui.add(egui::Slider::new(
                    &mut scene.settings.samples_per_pixel,
                    1..=10000,
                ));
                ui.label("Max ray depth:");
                ui.add(egui::Slider::new(&mut scene.settings.max_ray_depth, 1..=50));
                ui.horizontal(|ui| {
                    ui.label("Clamp:");
                    ui.add(egui::DragValue::new(&mut scene.settings.clamp_indirect).speed(0.5));
                });

                ui.horizontal(|ui| {
                    ui.label("Render mode:");
                    ComboBox::from_label("")
                        .selected_text(format!("{:?}", scene.settings.mode))
                        .show_ui(ui, |ui| {
                            if ui
                                .selectable_value(
                                    &mut scene.settings.mode,
                                    RenderMode::Full,
                                    "Full",
                                )
                                .clicked()
                            {
                                modified = true
                            };
                            if ui
                                .selectable_value(
                                    &mut scene.settings.mode,
                                    RenderMode::Clay,
                                    "Clay",
                                )
                                .clicked()
                            {
                                modified = true
                            };
                            if ui
                                .selectable_value(
                                    &mut scene.settings.mode,
                                    RenderMode::Normal,
                                    "Normal",
                                )
                                .clicked()
                            {
                                modified = true
                            };
                            if ui
                                .selectable_value(
                                    &mut scene.settings.mode,
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
                    &mut scene.settings.enable_multithreading,
                    "Enable multithreading",
                ));
                ui.add(egui::Checkbox::new(
                    &mut scene.settings.enable_bvh_tree,
                    "Enable Bvh tree",
                ));
                ui.add(egui::Checkbox::new(
                    &mut self.continuous_mode,
                    "Continuous mode",
                ));

                ui.separator();
                ui.heading("Scene Settings");
                ui.collapsing("Camera", |ui| {
                    scene.camera.display_ui(ui, &mut modified);
                });

                ui.collapsing("Background", |ui| {
                    scene.background.display_ui(ui, &mut modified);
                    ui.horizontal(|ui| {
                        ui.menu_button("Change background", |ui| {
                            if ui.button("Uniform background").clicked() {
                                scene.background =
                                    Box::new(UniformBackground::new(Color::new(0.8, 0.8, 0.8)));
                                ui.close_menu();
                                modified = true;
                            }
                            if ui.button("Gradient background").clicked() {
                                scene.background = Box::new(GradientBackground::new(
                                    Color::new(0.5, 0.7, 1.0),
                                    Color::new(1.0, 1.0, 1.0),
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
                    let now = Instant::now();
                    render(&mut *self.render_target.borrow_mut(), &self.scene);
                    self.last_time = Instant::now().duration_since(now);
                }

                ui.label(format!("Last render took: {:?}", self.last_time));
                ui.label(format!("Using {:?} threads", rayon::current_num_threads()));

                if modified && self.continuous_mode {
                    let now = Instant::now();
                    render(&mut *self.render_target.borrow_mut(), &self.scene);
                    self.last_time = Instant::now().duration_since(now);
                }
            });
    }
}
