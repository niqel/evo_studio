use vello::{Scene, kurbo::Affine, peniko::Fill};
use super::display::{DrawOp, TextOp, TextCommand};

pub struct Emitted {
    pub scene: Scene,
    pub texts: Vec<TextCommand>,
}

pub fn emit_scene_and_texts(
    ops: impl IntoIterator<Item = DrawOp>,
    _width: u32,
    _height: u32,
) -> Emitted {
    let mut scene = Scene::new();
    let mut texts = Vec::new();

    for op in ops {
        match op {
            DrawOp::FillRect { rect, color } => {
                scene.fill(Fill::NonZero, Affine::IDENTITY, color, None, &rect);
            }
            DrawOp::FillRoundedRect { rect, radius, color } => {
                // Usa RoundedRect real para esquinas redondeadas
                let rr = vello::kurbo::RoundedRect::from_rect(rect, radius);
                scene.fill(Fill::NonZero, Affine::IDENTITY, color, None, &rr);
            }
            DrawOp::Image { rect, key: _ } => {
                // TODO: rasterizar PNG por clave usando ImageCache; placeholder visual
                let rr = vello::kurbo::RoundedRect::from_rect(rect, 6.0);
                scene.fill(
                    Fill::NonZero,
                    Affine::IDENTITY,
                    vello::peniko::Color::from_rgba8(255, 255, 255, 200),
                    None,
                    &rr,
                );
            }
            DrawOp::Text(TextOp { rect, text, px, color, halign, valign }) => {
                texts.push(TextCommand { rect, text, px, color, halign, valign });
            }
        }
    }

    Emitted { scene, texts }
}

/// Compatibilidad: si quieres sólo la Scene (ignorando textos)
pub fn emit_scene(ops: impl IntoIterator<Item = DrawOp>, width: u32, height: u32) -> Scene {
    emit_scene_and_texts(ops, width, height).scene
}
