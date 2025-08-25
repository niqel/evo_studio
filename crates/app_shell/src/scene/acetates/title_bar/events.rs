//! events.rs — Stubs de eventos para TitleBar (sin vello).
//! Más adelante: drag de ventana, botones, menú, etc.

#![allow(dead_code)]

use super::component::TitleBar;

impl TitleBar {
    pub fn on_cursor_moved(&mut self, px: f64, py: f64) {
        self.ui_set_cursor((px, py));
    }

    pub fn on_mouse_down(&mut self, _px: f64, _py: f64) {
        // futuro: clicks en botones (cerrar, minimizar, etc.)
    }

    pub fn on_mouse_up(&mut self) {
        // futuro: no-op por ahora
    }
}
