use vello::kurbo::Rect;
use vello::peniko::Color;

#[derive(Clone, Copy, Debug)]
pub enum HAlign { Left, Center, Right }
#[derive(Clone, Copy, Debug)]
pub enum VAlign { Top, Middle, Bottom }

#[derive(Clone, Debug)]
pub enum DrawOp {
    FillRect { rect: Rect, color: Color },
    FillRoundedRect { rect: Rect, radius: f64, color: Color },
    /// Imagen raster (PNG) identificada por clave lógica
    Image { rect: Rect, key: &'static str },
    Text(TextOp),
}

#[derive(Clone, Debug)]
pub struct TextOp {
    pub rect: Rect,
    pub text: String,
    pub px: f32,            // tamaño en px
    pub color: Color,
    pub halign: HAlign,
    pub valign: VAlign,
}

/// Comando listo para el motor de texto (coordenadas en DIP)
#[derive(Clone, Debug)]
pub struct TextCommand {
    pub rect: Rect,
    pub text: String,
    pub px: f32,
    pub color: Color,
    pub halign: HAlign,
    pub valign: VAlign,
}
