// scene/state.rs — Estado global de UI que resulta de eventos (no-métrico).

#[derive(Debug)] // ← añade esto
pub struct UiToggles {
    sidebar: bool,
    panel: bool,
}

impl UiToggles {
    pub fn new(sidebar: bool, panel: bool) -> Self { Self { sidebar, panel } }
    pub fn default_on() -> Self { Self { sidebar: true, panel: true } }

    // Getters
    pub fn sidebar(&self) -> bool { self.sidebar }
    pub fn panel(&self)   -> bool { self.panel }

    // Mutación controlada (reacciones a eventos)
    pub fn set_sidebar(&mut self, v: bool) { self.sidebar = v; }
    pub fn set_panel(&mut self, v: bool)   { self.panel = v; }
    pub fn toggle_sidebar(&mut self) { self.sidebar = !self.sidebar; }
    pub fn toggle_panel(&mut self)   { self.panel  = !self.panel; }
}
