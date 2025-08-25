//! component.rs — Estado “vivo” del Panel (sin vello).

#[derive(Default)]
pub struct PanelUiState {
    pub(super) resizing: bool,          // futuro: drag del borde superior del panel
    pub(super) last_cursor: Option<(f64, f64)>,
}

pub struct Panel {
    pub(super) ui: PanelUiState,
}

impl Panel {
    pub fn new() -> Self {
        Self { ui: PanelUiState::default() }
    }

    // Azúcar para eventos/vista
    #[inline] pub fn ui_is_resizing(&self) -> bool { self.ui.resizing }
    #[inline] pub fn ui_start_resize(&mut self)    { self.ui.resizing = true; }
    #[inline] pub fn ui_stop_resize(&mut self)     { self.ui.resizing = false; }
    #[inline] pub fn ui_set_cursor(&mut self, p: (f64, f64)) { self.ui.last_cursor = Some(p); }
    #[inline] pub fn ui_last_cursor(&self) -> Option<(f64, f64)> { self.ui.last_cursor }
}

impl Default for Panel {
    fn default() -> Self { Self::new() }
}
