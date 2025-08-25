//! view.rs — Dibujo de la ActivityBar (único lugar que importa vello para la barra).

use vello::Scene;
use vello::kurbo::{Rect, Affine};
use vello::peniko::{Fill, Color};

use crate::scene::acetate::Acetate;
use crate::scene::colors;

use super::component::ActivityBar;
use super::layout::{self};
use super::component::ChromeSlot;
use super::actions::ActivityId;

// ✅ para la vista “pura” (display list)
use crate::sketcher::display::DrawOp;

impl ActivityBar {
    #[inline]
    fn draw_bar_background(&self, scene: &mut Scene, rect: Rect) {
        scene.fill(Fill::NonZero, Affine::IDENTITY, colors::ACTIVITY, None, &rect);
    }

    #[inline]
    fn draw_resize_handle(&self, scene: &mut Scene, handle: Rect) {
        let overlay = Color::from_rgba8(255, 255, 255, 10);
        scene.fill(Fill::NonZero, Affine::IDENTITY, overlay, None, &handle);
    }
}

// Implementación “legacy” de Acetate (stateless).
impl Acetate for ActivityBar {
    fn draw(&mut self, scene: &mut Scene, rect: Rect) {
        // 1) Fondo
        self.draw_bar_background(scene, rect);

        // 2) Layout puro (slots)
        let layout = layout::compute_layout(rect, &self.params, &self.top, &self.bottom);

        // 3) Ítems: dibujamos un chip/ícono placeholder centrado en cada slot
        for it_slot in &layout.items {
            let edge = (self.params.cell_h - 2.0 * self.params.padding).max(0.0);
            let w = edge;
            let h = edge;
            let x = it_slot.bounds.x0 + (it_slot.bounds.width() - w) * 0.5;
            let y = it_slot.bounds.y0 + self.params.padding;
            let icon_rect = Rect::new(x, y, x + w, y + h);

            // deshabilitado = más tenue
            let disabled = self.find_by_id(it_slot.id).map(|it| it.disabled).unwrap_or(false);
            let fill = if disabled {
                Color::from_rgba8(255, 255, 255, 80)
            } else {
                Color::WHITE
            };

            // No hay hover/active en legacy
            scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &icon_rect);
        }

        // 4) Handle de resize (visual opcional)
        if let Some(h) = layout.resize_handle {
            self.draw_resize_handle(scene, h);
        }
    }
}

// ----------------------
// Vistas “puras” (display list)
// ----------------------
impl ActivityBar {
    /// Vista pura parametrizada: recibe hovered/active externos (del Model).
    pub fn view_with(
        &self,
        rect: Rect,
        hovered: Option<ActivityId>,
        active: Option<ActivityId>,
    ) -> Vec<DrawOp> {
        let mut ops = Vec::new();

        // 1) Fondo de barra
        ops.push(DrawOp::FillRect { rect, color: colors::ACTIVITY });

        // 2) Layout puro (slots)
        let layout = layout::compute_layout(rect, &self.params, &self.top, &self.bottom);

        
        // 2.5) Chrome fijo: AppIcon, ArrowUp, ArrowDown, Settings
        for (slot, r) in &layout.chrome {
            match slot {
                ChromeSlot::AppIcon => {
                    // placeholder para PNG de app: por ahora un cuadrado blanco
                    ops.push(DrawOp::FillRoundedRect { rect: *r, radius: 8.0, color: Color::from_rgba8(255,255,255,255) });
                }
                ChromeSlot::ArrowUp => {
                    // flecha ▲ centrada + rayita superior
                    ops.push(DrawOp::Text(crate::sketcher::display::TextOp {
                        rect: *r, text: "▲".into(), px: 14.0, color: Color::WHITE,
                        halign: crate::sketcher::display::HAlign::Center,
                        valign: crate::sketcher::display::VAlign::Middle,
                    }));
                    let line_h = 2.0;
                    let lr = vello::kurbo::Rect::new(r.x0 + 6.0, r.y0 + 4.0, r.x1 - 6.0, r.y0 + 4.0 + line_h);
                    ops.push(DrawOp::FillRect { rect: lr, color: colors::SEPARATOR });
                }
                ChromeSlot::ArrowDown => {
                    // flecha ▼ centrada + rayita inferior
                    ops.push(DrawOp::Text(crate::sketcher::display::TextOp {
                        rect: *r, text: "▼".into(), px: 14.0, color: Color::WHITE,
                        halign: crate::sketcher::display::HAlign::Center,
                        valign: crate::sketcher::display::VAlign::Middle,
                    }));
                    let line_h = 2.0;
                    let lr = vello::kurbo::Rect::new(r.x0 + 6.0, r.y1 - 4.0 - line_h, r.x1 - 6.0, r.y1 - 4.0);
                    ops.push(DrawOp::FillRect { rect: lr, color: colors::SEPARATOR });
                }
                ChromeSlot::Settings => {
                    // engrane temporal con carácter unicode
                    ops.push(DrawOp::Text(crate::sketcher::display::TextOp {
                        rect: *r, text: "⚙".into(), px: 16.0, color: Color::WHITE,
                        halign: crate::sketcher::display::HAlign::Center,
                        valign: crate::sketcher::display::VAlign::Middle,
                    }));
                }
                ChromeSlot::ResizeHandle => {
                    ops.push(DrawOp::FillRect { rect: *r, color: Color::from_rgba8(255,255,255,20) });
                }
            }
        }
// 3) Ítems (overlays + icono “chip” redondeado)
        for it_slot in &layout.items {
            // Overlays hover/active
            if hovered == Some(it_slot.id) {
                ops.push(DrawOp::FillRect {
                    rect: it_slot.bounds,
                    color: Color::from_rgba8(255, 255, 255, 16),
                });
            }
            if active == Some(it_slot.id) {
                ops.push(DrawOp::FillRect {
                    rect: it_slot.bounds,
                    color: Color::from_rgba8(255, 255, 255, 28),
                });

                let strip_w = 3.0_f64;
                let strip = Rect::new(
                    it_slot.bounds.x0,
                    it_slot.bounds.y0,
                    (it_slot.bounds.x0 + strip_w).min(it_slot.bounds.x1),
                    it_slot.bounds.y1,
                );
                ops.push(DrawOp::FillRect { rect: strip, color: Color::WHITE });
            }

            // Icono placeholder (chip redondeado) centrado
            let edge = (self.params.cell_h - 2.0 * self.params.padding).max(0.0);
            let w = edge;
            let h = edge;
            let x = it_slot.bounds.x0 + (it_slot.bounds.width() - w) * 0.5;
            let y = it_slot.bounds.y0 + self.params.padding;
            let icon_rect = Rect::new(x, y, x + w, y + h);

            let disabled = self.find_by_id(it_slot.id).map(|it| it.disabled).unwrap_or(false);
            let fill = if disabled {
                Color::from_rgba8(255, 255, 255, 80)
            } else {
                Color::WHITE
            };
            ops.push(DrawOp::FillRoundedRect { rect: icon_rect, radius: 8.0, color: fill });
        }

        // 4) Handle de resize (visual opcional)
        if let Some(h) = layout.resize_handle {
            ops.push(DrawOp::FillRect {
                rect: h,
                color: Color::from_rgba8(255, 255, 255, 20),
            });
        }

        ops
    }

    /// Compatibilidad: versión sin estados (delegamos a `view_with(None,None)`).
    pub fn view(&self, rect: Rect) -> Vec<DrawOp> {
        self.view_with(rect, None, None)
    }
}
