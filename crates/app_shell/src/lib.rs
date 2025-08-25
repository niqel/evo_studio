// C:\repos\evo_studio\crates\app_shell\src\lib.rs

pub mod app;     // dueño del “modelo + update + puente con winit”
pub mod scene;   // puro: layout, view, tipos UI
pub mod sketcher; // <-- 💡 NECESARIO para que crate::sketcher exista
pub mod text;

pub use crate::app::App;

// (opcional) re-exportar para uso desde otros módulos/bin
// pub use sketcher::{Sketcher, SketchInput, SketchInputError, SketchOutput, DrawOp, emit_scene};
