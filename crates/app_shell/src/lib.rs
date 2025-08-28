// No expongas el módulo: privado hacia afuera
mod runner;
mod renderer;

// Reexporta SOLO el tipo en la raíz → app_shell::Runner
pub use runner::Runner;
pub use renderer::Renderer;
