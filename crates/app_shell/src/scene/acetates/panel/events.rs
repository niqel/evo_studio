//! events.rs — Stubs de eventos para Panel (sin vello).
//! Más adelante aquí manejaremos resize por borde, tabs del panel, etc.

#![allow(dead_code)]

use super::component::Panel;

impl Panel {
    pub fn on_cursor_moved(&mut self, px: f64, py: f64) {
        self.ui_set_cursor((px, py));
    }

    pub fn on_mouse_down(&mut self, _px: f64, _py: f64) {
        // futuro: detectar si se presionó el handle de resize (borde superior)
    }

    pub fn on_mouse_up(&mut self) {
        if self.ui_is_resizing() {
            self.ui_stop_resize();
        }
    }
}
