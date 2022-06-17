#![deny(clippy::all)]
#![forbid(unsafe_code)]

use crate::gui::Framework;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use ray_tracing_rust::render::RayTracingDemo;
use std::{cell::RefCell, rc::Rc};
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod gui;

const DEFAULT_SCENE: &str = "scenes/simple.json";

const RENDER_WIDTH: u32 = 500;
const RENDER_HEIGHT: u32 = 500;

const WINDOW_WIDTH: u32 = 2 * RENDER_WIDTH;
const WINDOW_HEIGHT: u32 = 2 * RENDER_HEIGHT;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
        WindowBuilder::new()
            .with_title("Ray Tracing Demo")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let app = Rc::new(RefCell::new(RayTracingDemo::load(
        RENDER_WIDTH,
        RENDER_HEIGHT,
        DEFAULT_SCENE,
    )));

    let (mut pixels, mut framework) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(RENDER_WIDTH, RENDER_HEIGHT, surface_texture)?;
        let framework = Framework::new(
            window_size.width,
            window_size.height,
            scale_factor,
            &pixels,
            Rc::clone(&app),
        );

        (pixels, framework)
    };

    event_loop.run(move |event, _, control_flow| {
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                framework.scale_factor(scale_factor);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
                framework.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }

        match event {
            Event::WindowEvent { event, .. } => {
                // Update egui inputs
                framework.handle_event(&event);
            }
            // Draw the current frame
            Event::RedrawRequested(_) => {
                // Draw the world
                if app.borrow().needs_redraw {
                    app.borrow_mut().draw(pixels.get_frame());
                }

                // Prepare egui
                framework.prepare(&window);

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    framework.render(encoder, render_target, context)?;

                    Ok(())
                });

                // Basic error handling
                if render_result
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
}
