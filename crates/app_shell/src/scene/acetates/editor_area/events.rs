//! events.rs — Stubs de eventos para EditorArea (sin vello).
//! De momento solo almacenamos la última posición del cursor.

#![allow(dead_code)]

use super::component::EditorArea;

impl EditorArea {
    pub fn on_cursor_moved(&mut self, px: f64, py: f64) {
        self.ui_set_cursor((px, py));
    }

    pub fn on_mouse_down(&mut self, _px: f64, _py: f64) {
        // futuro: focus, selección, caret, etc.
    }

    pub fn on_mouse_up(&mut self) {
        // futuro: terminar drag/selección
    }

    pub fn on_scroll(&mut self, _dx: f32, _dy: f32) {
        // futuro: scroll y sincronización con layout del documento
    }
}
