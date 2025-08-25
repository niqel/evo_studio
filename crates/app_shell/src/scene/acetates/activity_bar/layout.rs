//! layout.rs — Cálculo de layout para ActivityBar (con slots fijos y carrusel)

use vello::kurbo::Rect;

use super::component::{
    ActivityItem,
    ActivityParams,
    ActivityLayout,
    ItemSlot,
    ChromeSlot,
};

/// Ancho del handle de resize (franja vertical al borde derecho).
const HANDLE_W: f64 = 3.0;

/// Pequeño margen inferior de seguridad para no "rozar" el borde/StatusBar.
const BOTTOM_SAFETY: f64 = 0.5;

/// Calcula slots de items (carrusel) + chrome fijo (app, flechas, settings).
/// Las flechas SIEMPRE son visibles (estilo Edge).
pub(super) fn compute_layout(
    rect: Rect,
    params: &ActivityParams,
    top: &[ActivityItem],
    bottom: &[ActivityItem],
) -> ActivityLayout {
    let mut items: Vec<ItemSlot> = Vec::new();
    let mut chrome: Vec<(ChromeSlot, Rect)> = Vec::new();

    let h = params.cell_h.max(0.0);
    if h <= 0.0 || rect.height() <= 0.0 {
        return ActivityLayout { items, resize_handle: None, chrome };
    }

    // Unir todas las actividades en una sola lista de carrusel
    let mut all: Vec<&ActivityItem> = Vec::with_capacity(top.len() + bottom.len());
    all.extend(top.iter());
    all.extend(bottom.iter());

    // Reservar handle a la derecha si aplica
    let (ix0, ix1, resize_handle) = if rect.width() >= HANDLE_W {
        let hx0 = rect.x1 - HANDLE_W;
        let handle = Rect::new(hx0, rect.y0, rect.x1, rect.y1);
        (rect.x0, hx0, Some(handle))
    } else {
        (rect.x0, rect.x1, None)
    };

    // Slots fijos
    // App icon (arriba)
    let r_app = Rect::new(ix0, rect.y0, ix1, (rect.y0 + h).min(rect.y1));
    chrome.push((ChromeSlot::AppIcon, r_app));

    // ArrowUp (siempre) debajo del app icon
    let r_up = Rect::new(ix0, r_app.y1, ix1, (r_app.y1 + h).min(rect.y1));
    chrome.push((ChromeSlot::ArrowUp, r_up));

    // Settings (abajo)
    let r_settings = Rect::new(ix0, (rect.y1 - h).max(rect.y0), ix1, rect.y1);
    chrome.push((ChromeSlot::Settings, r_settings));

    // ArrowDown (siempre) arriba de settings
    let r_down = Rect::new(ix0, (r_settings.y0 - h).max(r_up.y1), ix1, r_settings.y0);
    chrome.push((ChromeSlot::ArrowDown, r_down));

    // Ventana visible del carrusel
    let visible_top = r_up.y1;
    let visible_bot = (r_down.y0).min(rect.y1 - BOTTOM_SAFETY);
    let avail_h = (visible_bot - visible_top).max(0.0);
    let vis_count = (avail_h / h).floor() as usize;

    let start = 0usize; // el scroll_offset lo maneja component/events (TODO)
    let end = (start + vis_count).min(all.len());

    let mut y = visible_top;
    for it in &all[start..end] {
        if y >= visible_bot { break; }
        let y1 = (y + h).min(visible_bot);
        items.push(ItemSlot { id: it.id, bounds: Rect::new(ix0, y, ix1, y1) });
        y = y1;
    }

    ActivityLayout { items, resize_handle, chrome }
}
