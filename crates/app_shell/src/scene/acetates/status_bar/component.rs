//! component.rs — Estado “vivo” del StatusBar (sin vello).
//! Mantén esto mínimo; más adelante puedes agregar mensajes, indicadores, etc.

#[derive(Default)]
pub struct StatusBarUiState {
    pub(super) last_cursor: Option<(f64, f64)>,
}

pub struct StatusBar {
    pub(super) ui: StatusBarUiState,
}

impl StatusBar {
    pub fn new() -> Self {
        Self { ui: StatusBarUiState::default() }
    }

    // Azúcar para eventos/vista
    #[inline] pub fn ui_set_cursor(&mut self, p: (f64, f64)) { self.ui.last_cursor = Some(p); }
    #[inline] pub fn ui_last_cursor(&self) -> Option<(f64, f64)> { self.ui.last_cursor }
}

impl Default for StatusBar {
    fn default() -> Self { Self::new() }
}
