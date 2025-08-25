// C:\repos\evo_studio\crates\app_shell\src\scene\acetates\activity_bar\hit.rs
use vello::kurbo::Rect;

use super::actions::ActivityId;
use super::events::MouseDownResult;
use super::component::ActivityBar;
use super::layout;

#[inline]
fn contains(r: &Rect, p: (f64, f64)) -> bool {
    let (x, y) = p;
    x >= r.x0 && x <= r.x1 && y >= r.y0 && y <= r.y1
}

/// Devuelve el item bajo el cursor (si hay).
pub fn hovered(bar: &ActivityBar, area: Rect, p: (f64, f64)) -> Option<ActivityId> {
    let l = layout::compute_layout(area, &bar.params, &bar.top, &bar.bottom);
    l.items.iter().find(|s| contains(&s.bounds, p)).map(|s| s.id)
}

/// Evalúa un mouse down: item, resize o none.
pub fn mouse_down(bar: &ActivityBar, area: Rect, p: (f64, f64)) -> MouseDownResult {
    let l = layout::compute_layout(area, &bar.params, &bar.top, &bar.bottom);
    if l.resize_handle.map_or(false, |h| contains(&h, p)) {
        return MouseDownResult::StartResize;
    }
    if let Some(id) = l.items.iter().find(|s| contains(&s.bounds, p)).map(|s| s.id) {
        MouseDownResult::Item(id)
    } else {
        MouseDownResult::None
    }
}
