//! view.rs — Dibujo del Panel (único lugar que importa vello).

use vello::Scene;
use vello::kurbo::{Rect, Affine};
use vello::peniko::Fill;

use crate::scene::acetate::Acetate;
use crate::scene::colors;

use super::component::Panel;

// ✅ para la vista “pura” (display list)
use crate::sketcher::display::DrawOp;

impl Acetate for Panel {
    fn draw(&mut self, scene: &mut Scene, rect: Rect) {
        // Fondo del panel inferior
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            colors::PANEL,
            None,
            &rect,
        );

        // Aquí irá el contenido del panel (consola, problemas, terminal, etc.)
    }
}

// ----------------------
// Vista “pura” (display list)
// ----------------------
impl Panel {
    /// Devuelve las operaciones de dibujo necesarias sin tocar `Scene`.
    /// Útil para el pipeline declarativo del `Sketcher`.
    pub fn view(rect: Rect) -> Vec<DrawOp> {
        vec![
            DrawOp::FillRect { rect, color: colors::PANEL },
        ]
    }
}
