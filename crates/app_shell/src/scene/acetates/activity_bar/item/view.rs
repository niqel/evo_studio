//! item/view.rs — Dibujo del ActivityItem dentro de un slot.

use vello::Scene;
use vello::kurbo::{Rect, RoundedRect, Affine};
use vello::peniko::{Fill, Color};

use super::super::layout::ActivityBarParams;
use super::component::ActivityItem;

impl ActivityItem {
    /// Dibuja el ítem dentro del `slot`. `params` define padding/tamaños.
    /// `is_hover` y `is_active` los decide el contenedor (ActivityBar).
    pub fn draw(&self, scene: &mut Scene, slot: Rect, params: &ActivityBarParams, is_hover: bool, is_active: bool) {
        // Overlays (hover/active)
        if is_hover {
            let overlay = Color::from_rgba8(255, 255, 255, 16);
            scene.fill(Fill::NonZero, Affine::IDENTITY, overlay, None, &slot);
        }
        if is_active {
            let overlay = Color::from_rgba8(255, 255, 255, 28);
            scene.fill(Fill::NonZero, Affine::IDENTITY, overlay, None, &slot);

            let strip_w = 3.0;
            let active_strip = Rect::new(slot.x0, slot.y0, (slot.x0 + strip_w).min(slot.x1), slot.y1);
            scene.fill(Fill::NonZero, Affine::IDENTITY, Color::WHITE, None, &active_strip);
        }

        // Icono placeholder (chip redondeado) centrado horizontalmente con padding superior.
        let icon_edge = (params.cell_h - 2.0 * params.padding).max(0.0);
        let w = icon_edge;
        let h = icon_edge;
        let x = slot.x0 + (slot.width() - w) * 0.5;
        let y = slot.y0 + params.padding;
        let icon_rect = Rect::new(x, y, x + w, y + h);

        let rr = RoundedRect::from_rect(icon_rect, 8.0);
        let fill = if self.disabled { Color::from_rgba8(255, 255, 255, 80) } else { Color::WHITE };
        scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &rr);
    }
}
