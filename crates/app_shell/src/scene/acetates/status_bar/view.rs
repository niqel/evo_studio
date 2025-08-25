//! view.rs — display-list para StatusBar (texto + fondo), sin tocar vello aquí.

use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::scene::colors;
use crate::sketcher::display::{DrawOp, TextOp, HAlign, VAlign};

use super::component::StatusBar;

impl StatusBar {
    /// Vista pura: devuelve las ops de dibujo (fondo + texto).
    /// Es estático para que puedas usarlo desde el Sketcher con `.chain(...)`.
    pub fn view(rect: Rect) -> impl Iterator<Item = DrawOp> {
        // Ajusta el label si quieres mostrar algo real del estado
        let label = "UTF-8   LF   Rust   Ready";
        let text_px = 12.0_f32;

        [
            // Fondo de la status bar. Si tu paleta usa `STATUS_BG`, cámbialo aquí.
            DrawOp::FillRect { rect, color: colors::STATUS },

            // Texto alineado a la derecha y centrado verticalmente
            DrawOp::Text(TextOp {
                rect,
                text: label.to_string(),
                px: text_px,
                color: Color::from_rgba8(230, 230, 230, 255),
                halign: HAlign::Right,
                valign: VAlign::Middle,
            }),
        ]
        .into_iter()
    }
}
