pub mod selection;

use selection::Rect; // Direction,
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

struct App {
    window: Option<Box<dyn Window>>,
    rect: Rect,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            rect: Rect::new(0, 0, 100, 100),
        }
    }
}

impl ApplicationHandler for App {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title("Keyshot")
            .with_transparent(true)
            .with_decorations(false);

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
