use vello::wgpu;

/// Comando de texto que viene del Sketcher
use crate::sketcher::display::TextCommand;

/// Motor de texto (placeholder). Implementaremos glyphon aquí después.
pub struct TextEngine {
    _format: wgpu::TextureFormat,
}

impl TextEngine {
    pub fn new(format: wgpu::TextureFormat, _device: &wgpu::Device, _queue: &wgpu::Queue) -> Self {
        Self { _format: format }
    }

    /// Dibuja texto sobre `target_view` (NO-OP de momento).
    #[allow(unused_variables)]
    pub fn render(
        &mut self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _encoder: &mut wgpu::CommandEncoder,
        _target_view: &wgpu::TextureView,
        _width: u32,
        _height: u32,
        _texts: &[TextCommand],
    ) {
        // TODO: integrar glyphon aquí (prepare + render)
    }
}
