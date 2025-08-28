// crates/app_shell/src/renderer/renderer.rs

use super::snapshot::Snapshot;

/// Modificadores “puros” (sin depender de winit)
#[derive(Debug, Clone, Copy, Default)]
pub struct KeyModifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub logo: bool, // ⊞ / ⌘
}

/// Tecla lógica “pura”
#[derive(Debug, Clone)]
pub enum LogicalKey {
    Char(char),
    Escape,
    Enter,
    Tab,
    Backspace,
    Space,
    Named(String),
    None,
}

/// Input de teclado (puro)
#[derive(Debug, Clone)]
pub struct KeyInput {
    pub is_pressed: bool,
    pub logical: LogicalKey,
    pub modifiers: KeyModifiers,
}

#[derive(Debug, Default)]
pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Self
    }

    // -------------------- Wrappers de dominio Snapshot --------------------

    pub fn snapshot_initial(
        &self,
        viewport_width: u32,
        viewport_height: u32,
        clear_rgba: (u8, u8, u8, u8),
    ) -> Snapshot {
        Snapshot::initial(viewport_width, viewport_height).with_clear_rgba(clear_rgba)
    }

    pub fn snapshot_resize(
        &self,
        snap: Snapshot,
        viewport_width: u32,
        viewport_height: u32,
    ) -> Snapshot {
        snap.with_viewport(viewport_width, viewport_height)
    }

    pub fn snapshot_set_clear(&self, snap: Snapshot, rgba: (u8, u8, u8, u8)) -> Snapshot {
        snap.with_clear_rgba(rgba)
    }

    // -------------------- Inputs puros desde Runner -----------------------

    pub fn on_cursor_moved(prev: Snapshot, x: f64, y: f64) -> Snapshot {
        prev.with_cursor_position(x, y)
    }

    pub fn on_mouse_down_left(prev: Snapshot, _x: f64, _y: f64) -> Snapshot {
        prev.with_clear_rgba((255, 0, 0, 255))
    }

    pub fn on_mouse_up_left(prev: Snapshot, _x: f64, _y: f64) -> Snapshot {
        prev.with_clear_rgba((0, 0, 255, 255))
    }

    pub fn on_key_input(prev: Snapshot, input: KeyInput) -> Snapshot {
        if !input.is_pressed {
            return prev;
        }
        match input.logical {
            LogicalKey::Char('r') | LogicalKey::Char('R') => prev.with_clear_rgba((255, 0, 0, 255)),
            LogicalKey::Char('g') | LogicalKey::Char('G') => prev.with_clear_rgba((0, 255, 0, 255)),
            LogicalKey::Char('b') | LogicalKey::Char('B') => prev.with_clear_rgba((0, 0, 255, 255)),
            _ => prev,
        }
    }

    // -------------------- Render (puro / mock) ----------------------------

    pub fn render(&self, prev: Snapshot) -> Snapshot {
        let next = prev.advanced(0.016, false);

        let cursor_str = match next.cursor_position() {
            Some((cx, cy)) => format!("({:.1}, {:.1})", cx, cy),
            None => "none".to_string(),
        };

        println!(
            "render → viewport={}x{}, clear=({:?}), t={:.3}, anim={}, cursor={}",
            next.viewport_width(),
            next.viewport_height(),
            next.clear_rgba(),
            next.time_seconds(),
            next.has_active_animations(),
            cursor_str,
        );

        next
    }
}
