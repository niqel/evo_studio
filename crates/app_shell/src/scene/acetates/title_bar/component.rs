//! component.rs — Estado “vivo” del TitleBar (sin vello).
//! Aquí luego podrás guardar título del proyecto, botones, etc.

#[derive(Default)]
pub struct TitleBarUiState {
    pub(super) last_cursor: Option<(f64, f64)>,
}

pub struct TitleBar {
    pub(super) ui: TitleBarUiState,
}

impl TitleBar {
    pub fn new() -> Self {
        Self { ui: TitleBarUiState::default() }
    }

    // Azúcar para eventos/vista
    #[inline] pub fn ui_set_cursor(&mut self, p: (f64, f64)) { self.ui.last_cursor = Some(p); }
    #[inline] pub fn ui_last_cursor(&self) -> Option<(f64, f64)> { self.ui.last_cursor }
}

impl Default for TitleBar {
    fn default() -> Self { Self::new() }
}
