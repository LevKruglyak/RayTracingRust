use egui::{panel::Side, ClippedMesh, Context, TexturesDelta};
use egui_wgpu_backend::{BackendError, RenderPass, ScreenDescriptor};
use pixels::{wgpu, PixelsContext};
use ray_tracing_rust::render::RayTracingDemo;
use std::{cell::RefCell, rc::Rc};
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
        app: Rc<RefCell<RayTracingDemo>>,
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
        let gui = Gui::new(app);

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
    app: Rc<RefCell<RayTracingDemo>>,
}

impl Gui {
    /// Create a `Gui`.
    fn new(app: Rc<RefCell<RayTracingDemo>>) -> Self {
        Self { app }
    }

    /// Create the UI using egui.
    fn ui(&mut self, ctx: &Context) {
        egui::Window::new("Render Settings")
            .open(&mut true)
            .show(ctx, |ui| {
                let mut app = self.app.borrow_mut();
                ui.label("Samples per pixel:");
                ui.add(egui::Slider::new(
                    &mut app.scene.settings.samples_per_pixel,
                    1..=200,
                ));
                ui.label("Max ray depth:");
                ui.add(egui::Slider::new(
                    &mut app.scene.settings.max_ray_depth,
                    1..=50,
                ));

                ui.label("Field of view:");
                ui.add(egui::Slider::new(
                    &mut app.scene.camera.vertical_fov,
                    0.60..=120.0,
                ));
                ui.add(egui::Checkbox::new(
                    &mut app.scene.settings.enable_multithreading,
                    "Enable multithreading",
                ));

                ui.separator();
                if ui.button("Render Image").clicked() {
                    app.update();
                }

                ui.label(format!("Last render took: {:?}", app.last_time));
                ui.label(format!("Using {:?} threads", rayon::current_num_threads()));
            });
    }
}
