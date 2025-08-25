#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use app_shell::App; // gracias al `pub use` en lib.rs
use winit::{error::EventLoopError, event_loop::EventLoop};

fn main() -> Result<(), EventLoopError> {
    // Opción A (si tu winit requiere 'static): Box::leak es válido
    EventLoop::new()
        .and_then(|event_loop| event_loop.run_app(Box::leak(Box::new(App::new()))))

    // Opción B (si tu run_app acepta &mut y no 'static):
    // let event_loop = EventLoop::new()?;
    // let mut app = App::new();
    // event_loop.run_app(&mut app)
}
