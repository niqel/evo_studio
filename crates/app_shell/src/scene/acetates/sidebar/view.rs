//! view.rs — Dibujo del SideBar (único lugar que importa vello).

use vello::Scene;
use vello::kurbo::{Rect, Affine};
use vello::peniko::Fill;

use crate::scene::acetate::Acetate;
use crate::scene::colors;

use super::component::SideBar;

// ✅ para la vista “pura” (display list)
use crate::sketcher::display::DrawOp;

impl Acetate for SideBar {
    fn draw(&mut self, scene: &mut Scene, rect: Rect) {
        // Fondo de la barra lateral
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            colors::SIDEBAR,
            None,
            &rect,
        );

        // Aquí irá el contenido del Sidebar (árbol, búsqueda, etc.)
    }
}

// ----------------------
// Vista “pura” (display list)
// ----------------------
impl SideBar {
    /// Devuelve las operaciones de dibujo necesarias sin tocar `Scene`.
    /// Útil para el pipeline declarativo del `Sketcher`.
    pub fn view(rect: Rect) -> Vec<DrawOp> {
        vec![
            DrawOp::FillRect { rect, color: colors::SIDEBAR },
        ]
    }
}
