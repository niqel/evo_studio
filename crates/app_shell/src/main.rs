#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

fn main() -> Result<(), winit::error::EventLoopError> {
    app_shell::Runner::run()
}
