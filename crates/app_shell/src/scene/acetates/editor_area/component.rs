//! component.rs — Estado “vivo” del EditorArea (sin vello).

#[derive(Default)]
pub struct EditorUiState {
    pub(super) last_cursor: Option<(f64, f64)>,
}

pub struct EditorArea {
    pub(super) ui: EditorUiState,
}

impl EditorArea {
    pub fn new() -> Self {
        Self { ui: EditorUiState::default() }
    }

    // Azúcar para la vista/eventos
    #[inline] pub fn ui_last_cursor(&self) -> Option<(f64, f64)> { self.ui.last_cursor }
    #[inline] pub fn ui_set_cursor(&mut self, p: (f64, f64)) { self.ui.last_cursor = Some(p); }
}

impl Default for EditorArea {
    fn default() -> Self { Self::new() }
}
