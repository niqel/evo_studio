//! item/events.rs — Reacciones locales del ActivityItem (stubs).

use super::component::ActivityItem;

impl ActivityItem {
    pub fn on_hover_enter(&mut self) {
        // futuro: prefetch, anim, tooltip timing, etc.
    }
    pub fn on_hover_leave(&mut self) {
        // futuro: cleanup hover state/anim
    }
    pub fn on_click(&mut self) {
        // futuro: feedback / anim / analytics
    }
}
