#![deny(clippy::all)]
#![forbid(unsafe_code)]

use crate::gui::Framework;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use ray_tracing_rust::core::render::RenderTarget;
use std::cell::RefCell;
use std::rc::Rc;
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod gui;

const SCALE_DOWN: u32 = 2;

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 1000;

const RENDER_WIDTH: u32 = WINDOW_WIDTH / SCALE_DOWN;
const RENDER_HEIGHT: u32 = WINDOW_HEIGHT / SCALE_DOWN;

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

    let target = Rc::new(RefCell::new(RenderTarget::new(
        RENDER_WIDTH as usize,
        RENDER_HEIGHT as usize,
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
            Rc::clone(&target),
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
                if target.borrow().request_redraw {
                    pixels
                        .get_frame()
                        .copy_from_slice(&target.borrow().data[..]);
                    target.borrow_mut().request_redraw = false;
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
