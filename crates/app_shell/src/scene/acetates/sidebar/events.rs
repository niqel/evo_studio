//! events.rs — Stubs de eventos para SideBar (sin vello).
//! Más adelante: manejo de resize por divisor, selección en árbol, etc.

#![allow(dead_code)]

use super::component::SideBar;

impl SideBar {
    pub fn on_cursor_moved(&mut self, px: f64, py: f64) {
        self.ui_set_cursor((px, py));
    }

    pub fn on_mouse_down(&mut self, _px: f64, _py: f64) {
        // futuro: detectar si se presionó el divisor de resize
    }

    pub fn on_mouse_up(&mut self) {
        if self.ui_is_resizing() {
            self.ui_stop_resize();
        }
    }
}
