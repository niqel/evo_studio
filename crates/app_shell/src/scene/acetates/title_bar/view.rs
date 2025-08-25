use vello::kurbo::Rect;
use vello::peniko::Color;

use crate::scene::colors;
use crate::scene::acetates::title_bar::component::TitleBar;
use crate::sketcher::display::{DrawOp, TextOp, HAlign, VAlign};

impl TitleBar {
    /// Dibuja la barra de título ocupando el `rect` recibido.
    /// Devuelve un iterador de operaciones de dibujo para encadenar con el resto de la UI.
    pub fn view(rect: Rect) -> impl Iterator<Item = DrawOp> {
        let title = "evo_studio"; // ajusta si lo tomas de estado/app
        let text_px = 13.0_f32;

        [
            // Fondo de la TitleBar
            DrawOp::FillRect { rect, color: colors::TITLE_BG },
            // Texto (alineado a la izquierda, centrado verticalmente)
            DrawOp::Text(TextOp {
                rect,
                text: title.to_string(),
                px: text_px,
                color: Color::WHITE,
                halign: HAlign::Left,
                valign: VAlign::Middle,
            }),
        ]
        .into_iter()
    }
}
