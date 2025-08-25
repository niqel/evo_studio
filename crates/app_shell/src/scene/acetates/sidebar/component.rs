//! component.rs — Estado “vivo” del SideBar (sin vello).

#[derive(Default)]
pub struct SideBarUiState {
    pub(super) resizing: bool,                 // futuro: drag del divisor con el editor
    pub(super) last_cursor: Option<(f64, f64)>,
}

pub struct SideBar {
    pub(super) ui: SideBarUiState,
}

impl SideBar {
    pub fn new() -> Self {
        Self { ui: SideBarUiState::default() }
    }

    // Azúcar para eventos/vista
    #[inline] pub fn ui_is_resizing(&self) -> bool { self.ui.resizing }
    #[inline] pub fn ui_start_resize(&mut self)    { self.ui.resizing = true; }
    #[inline] pub fn ui_stop_resize(&mut self)     { self.ui.resizing = false; }
    #[inline] pub fn ui_set_cursor(&mut self, p: (f64, f64)) { self.ui.last_cursor = Some(p); }
    #[inline] pub fn ui_last_cursor(&self) -> Option<(f64, f64)> { self.ui.last_cursor }
}

impl Default for SideBar {
    fn default() -> Self { Self::new() }
}
