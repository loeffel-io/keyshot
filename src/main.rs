pub mod selection;

use selection::{Direction, Rect};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::monitor::Fullscreen;
use winit::window::{Window, WindowAttributes, WindowId};

pub enum Mode {
    Positioning,
    Selecting,
    Finished,
}

struct App {
    window: Option<Box<dyn Window>>,
    rect: Rect,
    mode: Mode,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
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
            .with_transparent(false)
            .with_decorations(true)
            .with_fullscreen(Some(Fullscreen::Borderless(None)));

        self.window = Some(event_loop.create_window(window_attributes).unwrap());
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
                let step = 10;

                match keycode {
                    KeyCode::KeyH | KeyCode::ArrowLeft => {
                        println!("H");
                        self.handle_movement(Direction::Left, step);
                    }
                    KeyCode::KeyJ | KeyCode::ArrowDown => {
                        println!("J");
                        self.handle_movement(Direction::Down, step);
                    }
                    KeyCode::KeyK | KeyCode::ArrowUp => {
                        println!("K");
                        self.handle_movement(Direction::Up, step);
                    }
                    KeyCode::KeyL | KeyCode::ArrowRight => {
                        println!("L");
                        self.handle_movement(Direction::Right, step);
                    }
                    KeyCode::Space => {
                        println!("Space");
                    }
                    KeyCode::Escape => {
                        println!("Escape");
                    }
                    KeyCode::Enter => {
                        println!("Enter");
                    }
                    _ => (),
                }
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello World!");

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
