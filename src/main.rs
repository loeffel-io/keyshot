pub mod selection;

use pixels::{Pixels, SurfaceTexture};
use selection::{Direction, Rect};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowLevel;
use winit::window::{Window, WindowAttributes, WindowId};

pub enum Mode {
    Positioning,
    Selecting,
    Finished,
}

struct App {
    window: Option<Arc<dyn Window>>,
    pixels: Option<Pixels<'static>>,
    rect: Rect,
    mode: Mode,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            pixels: None,
            rect: Rect::new(0, 0, 100, 100),
            mode: Mode::Positioning,
        }
    }

    fn handle_movement(&mut self, direction: Direction, distance: i32) {
        match self.mode {
            Mode::Positioning => {
                println!("Positioning mode: moving selection");
                self.rect.move_selection(direction, distance);
                println!("New rect position: x={}, y={}", self.rect.x, self.rect.y);
            }
            Mode::Selecting => {
                println!("Selecting mode: moving selection");
            }
            Mode::Finished => {
                println!("Finished mode: no movement allowed");
            }
        }
    }
}

impl ApplicationHandler for App {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title("Keyshot")
            .with_transparent(true)
            .with_decorations(false)
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_maximized(true);

        let window: Arc<dyn Window> =
            Arc::from(event_loop.create_window(window_attributes).unwrap());

        let size = window.surface_size();

        let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());

        let pixels = pixels::PixelsBuilder::new(size.width, size.height, surface_texture)
            .clear_color(pixels::wgpu::Color::TRANSPARENT)
            .alpha_mode(pixels::wgpu::CompositeAlphaMode::PostMultiplied)
            .build()
            .unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(keycode),
                        ..
                    },
                ..
            } => {
                let step = 50;

                match keycode {
                    KeyCode::KeyH | KeyCode::ArrowLeft => {
                        self.handle_movement(Direction::Left, step);
                    }
                    KeyCode::KeyJ | KeyCode::ArrowDown => {
                        self.handle_movement(Direction::Down, step);
                    }
                    KeyCode::KeyK | KeyCode::ArrowUp => {
                        self.handle_movement(Direction::Up, step);
                    }
                    KeyCode::KeyL | KeyCode::ArrowRight => {
                        self.handle_movement(Direction::Right, step);
                    }
                    KeyCode::Space => {
                        println!("Space");
                        // toggle between positioning and selecting mode
                        self.mode = match self.mode {
                            Mode::Positioning => Mode::Selecting,
                            Mode::Selecting => Mode::Positioning,
                            Mode::Finished => Mode::Finished,
                        };
                    }
                    KeyCode::Escape => {
                        println!("Escape");
                        event_loop.exit();
                    }
                    KeyCode::Enter => {
                        println!("Enter");
                        self.mode = Mode::Finished;
                        event_loop.exit();
                    }
                    _ => (),
                }

                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
            }
            WindowEvent::SurfaceResized(size) => {
                if size.width > 0 && size.height > 0 {
                    if let Some(pixels) = self.pixels.as_mut() {
                        pixels.resize_surface(size.width, size.height).unwrap();
                        pixels.resize_buffer(size.width, size.height).unwrap();
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                let (Some(window), Some(pixels)) = (self.window.as_ref(), self.pixels.as_mut())
                else {
                    return;
                };

                let size = window.surface_size();
                if size.width == 0 || size.height == 0 {
                    return;
                }

                let frame = pixels.frame_mut();

                // 1. clear frame to be completely transparent (R=0, G=0, B=0, A=0)
                for pixel in frame.chunks_exact_mut(4) {
                    pixel.copy_from_slice(&[0, 0, 0, 0]);
                }

                let width = size.width as usize;
                let height = size.height as usize;

                // safely cast rect dimensions
                let r_x = self.rect.x.max(0) as usize;
                let r_y = self.rect.y.max(0) as usize;
                let r_w = self.rect.width as usize;
                let r_h = self.rect.height as usize;

                // 2. define the RGBA green border (A=255 is fully opaque)
                let green = [0, 255, 0, 255];

                // 3. highly optimized border drawing
                // draw top and bottom lines
                for x in r_x..=(r_x + r_w).min(width.saturating_sub(1)) {
                    if r_y < height {
                        let i = (r_y * width + x) * 4;
                        frame[i..i + 4].copy_from_slice(&green);
                    }
                    if r_y + r_h < height {
                        let i = ((r_y + r_h) * width + x) * 4;
                        frame[i..i + 4].copy_from_slice(&green);
                    }
                }

                // draw left and right lines
                for y in r_y..=(r_y + r_h).min(height.saturating_sub(1)) {
                    if r_x < width {
                        let i = (y * width + r_x) * 4;
                        frame[i..i + 4].copy_from_slice(&green);
                    }
                    if r_x + r_w < width {
                        let i = (y * width + r_x + r_w) * 4;
                        frame[i..i + 4].copy_from_slice(&green);
                    }
                }

                // 4. push the frame to the actual screen
                pixels.render().unwrap();
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(App::new())?;
    Ok(())
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> i32 {
    main();
    0
}
