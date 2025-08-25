pub mod domain;
pub mod codomain;

// Nuevo: display list + emisor
pub mod display;
mod emitter;

mod sketcher;

pub use sketcher::Sketcher;
pub use domain::{SketchInput, SketchInputError};
pub use codomain::SketchOutput;

pub use display::DrawOp;
pub use emitter::emit_scene;
// 👇 añade esta re-exportación
pub use emitter::emit_scene_and_texts;
