//! view.rs — Dibujo del EditorArea (único lugar que importa vello).

use vello::Scene;
use vello::kurbo::{Rect, Affine};
use vello::peniko::Fill;

use crate::scene::acetate::Acetate;
use crate::scene::colors;

use super::component::EditorArea;

// ✅ para la vista “pura” (display list)
use crate::sketcher::display::DrawOp;

impl Acetate for EditorArea {
    fn draw(&mut self, scene: &mut Scene, rect: Rect) {
        // Fondo del área del editor
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            colors::EDITOR,
            None,
            &rect,
        );

        // Aquí irá el resto del render (gutter, tabs, text, cursor, etc.)
    }
}

// ----------------------
// Vista “pura” (display list)
// ----------------------
impl EditorArea {
    /// Devuelve las operaciones de dibujo necesarias sin tocar `Scene`.
    /// Útil para el pipeline declarativo del `Sketcher`.
    pub fn view(rect: Rect) -> Vec<DrawOp> {
        vec![
            DrawOp::FillRect { rect, color: colors::EDITOR },
        ]
    }
}
