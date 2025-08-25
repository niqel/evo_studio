use kurbo::Rect;
use super::metrics::UiMetrics;
use super::state::UiToggles;

/// Layout global del shell.
/// Todos los rects están en px lógicos (DIP).
#[derive(Clone, Copy, Debug)]
pub struct UiLayout {
    pub title: Rect,
    pub activity: Rect,
    pub status: Rect,
    pub panel: Rect,
    pub sidebar: Rect,
    pub editor: Rect,
}

// Helpers simples
fn fw(width: u32) -> f64 { width as f64 }
fn fh(height: u32) -> f64 { height as f64 }

#[inline]
fn zero_rect() -> Rect { Rect::new(0.0, 0.0, 0.0, 0.0) }

/// Calcula el layout global a partir del tamaño de ventana, las métricas (DIP) y los toggles.
/// Regla de composición:
/// - TitleBar arriba (full width)
/// - StatusBar abajo (full width)
/// - ActivityBar a la izquierda (entre title y status)
/// - SideBar a la derecha de ActivityBar (entre title y status), sólo si está visible
/// - Editor ocupa el espacio restante entre title y status, y a la derecha de activity/sidebar
/// - Panel ocupa la franja inferior **del área de editor** (no cubre activity/sidebar); sólo si está visible
pub fn compute_layout(width: u32, height: u32, metrics: &UiMetrics, toggles: &UiToggles) -> UiLayout {
    let w = fw(width);
    let h = fh(height);

    // Espesores (DIP)
    let title_h   = metrics.titlebar_height().max(0.0);
    let status_h  = metrics.statusbar_height().max(0.0);
    let act_w     = metrics.activitybar_width().max(0.0);
    let sb_w      = if toggles.sidebar() { metrics.sidebar_width().max(0.0) } else { 0.0 };
    let panel_h   = if toggles.panel()   { metrics.panel_height().max(0.0)   } else { 0.0 };

    // Bandas horizontales (top/bottom)
    let top_y    = title_h.min(h);
    let bottom_y = (h - status_h).max(top_y);

    // Title y Status (full width)
    let title  = Rect::new(0.0, 0.0, w, top_y);
    let status = Rect::new(0.0, bottom_y, w, h);

    // ActivityBar (columna izquierda entre title y status)
    let act_x1 = act_w.min(w);
    let activity = Rect::new(0.0, top_y, act_x1, bottom_y);

    // Sidebar (si visible): a la derecha de ActivityBar, entre title y status
    let sb_x0 = act_x1;
    let sb_x1 = (sb_x0 + sb_w).min(w);
    let sidebar = if sb_w > 0.0 && sb_x1 > sb_x0 {
        Rect::new(sb_x0, top_y, sb_x1, bottom_y)
    } else {
        zero_rect()
    };

    // Editor: el espacio restante horizontal entre title y status
    let editor_left = if sb_w > 0.0 { sb_x1 } else { act_x1 };
    let editor_top  = top_y;
    let editor_bot  = (bottom_y - panel_h).max(editor_top);
    let editor = if editor_left < w && editor_bot > editor_top {
        Rect::new(editor_left, editor_top, w, editor_bot)
    } else {
        zero_rect()
    };

    // Panel: franja inferior **del área de editor** (si visible)
    let panel = if panel_h > 0.0 {
        let p_top = editor_bot;
        let p_bot = bottom_y;
        if p_bot > p_top && editor_left < w {
            Rect::new(editor_left, p_top, w, p_bot)
        } else {
            zero_rect()
        }
    } else {
        zero_rect()
    };

    UiLayout { title, activity, status, panel, sidebar, editor }
}
