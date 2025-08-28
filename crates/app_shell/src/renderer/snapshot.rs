// crates/app_shell/src/renderer/snapshot.rs

#[derive(Debug, Clone)]
pub struct Snapshot {
    // --- Invariantes del viewport ---
    viewport_width: u32,  // >= 1
    viewport_height: u32, // >= 1

    // --- Tiempo lógico (segundos) ---
    time_seconds: f64, // monotónico, no negativo

    // --- Señales de orquestación ---
    has_active_animations: bool,

    // --- Config visual mínima ---
    clear_rgba: (u8, u8, u8, u8),

    // --- Cursor (opcional) ---
    cursor_known: bool,
    cursor_x: f64,
    cursor_y: f64,
}

impl Snapshot {
    pub(super) fn initial(viewport_width: u32, viewport_height: u32) -> Self {
        let vw = viewport_width.max(1);
        let vh = viewport_height.max(1);
        Self {
            viewport_width: vw,
            viewport_height: vh,
            time_seconds: 0.0,
            has_active_animations: false,
            clear_rgba: (0, 0, 0, 255),
            cursor_known: false,
            cursor_x: 0.0,
            cursor_y: 0.0,
        }
    }

    pub(super) fn advanced(mut self, dt_seconds: f64, animating: bool) -> Self {
        let dt = dt_seconds.max(0.0);
        self.time_seconds += dt;
        self.has_active_animations = animating;
        self
    }

    pub(super) fn with_viewport(mut self, viewport_width: u32, viewport_height: u32) -> Self {
        self.viewport_width = viewport_width.max(1);
        self.viewport_height = viewport_height.max(1);
        self
    }

    pub(super) fn with_clear_rgba(mut self, rgba: (u8, u8, u8, u8)) -> Self {
        self.clear_rgba = rgba;
        self
    }

    // NUEVO: fijar posición de cursor (solo renderer puede llamarlo)
    pub(super) fn with_cursor_position(mut self, x: f64, y: f64) -> Self {
        self.cursor_known = true;
        self.cursor_x = x;
        self.cursor_y = y;
        self
    }

    // Getters públicos
    pub fn viewport_width(&self) -> u32 { self.viewport_width }
    pub fn viewport_height(&self) -> u32 { self.viewport_height }
    pub fn time_seconds(&self) -> f64 { self.time_seconds }
    pub fn has_active_animations(&self) -> bool { self.has_active_animations }
    pub fn clear_rgba(&self) -> (u8, u8, u8, u8) { self.clear_rgba }

    // Observación del cursor
    pub fn cursor_position(&self) -> Option<(f64, f64)> {
        if self.cursor_known { Some((self.cursor_x, self.cursor_y)) } else { None }
    }
}
