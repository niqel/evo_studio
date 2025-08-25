//! component.rs — Modelo estático de la ActivityBar (sin estado UI).

use vello::kurbo::Rect;

use super::actions::{ActivityAction, ActivityId};

/// Parámetros de medida/estética para el layout de items.
#[derive(Clone, Debug)]
pub struct ActivityParams {
    pub padding: f64,
    pub cell_h: f64,
}

impl Default for ActivityParams {
    fn default() -> Self {
        Self {
            padding: 6.0,
            cell_h: 42.0,
        }
    }
}

/// Ítem de ActivityBar (modelo declarativo).
#[derive(Clone, Debug)]
pub struct ActivityItem {
    pub id: ActivityId,
    pub action: ActivityAction,
    pub disabled: bool,
}

impl ActivityItem {
    pub fn new(id: ActivityId, action: ActivityAction) -> Self {
        Self { id, action, disabled: false }
    }
}

/// ActivityBar sin estado de UI; sólo catálogo de items + params.
#[derive(Clone, Debug)]
pub struct ActivityBar {
    pub params: ActivityParams,
    pub top:    Vec<ActivityItem>,
    pub bottom: Vec<ActivityItem>,
}

impl ActivityBar {
    pub fn new() -> Self {
        use super::actions::ActivityAction::*;
        // Catálogo por defecto (puedes ajustar IDs/orden)
        let top = vec![
            ActivityItem::new(ActivityId(1), ShowExplorer),
            ActivityItem::new(ActivityId(2), ShowSearch),
            ActivityItem::new(ActivityId(3), ShowSourceControl),
            ActivityItem::new(ActivityId(4), ShowRun),
            ActivityItem::new(ActivityId(5), ShowExtensions),
        ];
        let bottom = vec![
            ActivityItem::new(ActivityId(99), ShowSettings),
        ];
        Self { params: ActivityParams::default(), top, bottom }
    }

    #[inline]
    pub fn find_by_id(&self, id: ActivityId) -> Option<&ActivityItem> {
        self.top.iter().chain(self.bottom.iter()).find(|it| it.id == id)
    }

    #[inline]
    pub fn action_of(&self, id: ActivityId) -> Option<&ActivityAction> {
        self.find_by_id(id).map(|it| &it.action)
    }
}

// ===== API auxiliar opcional para layout =====

#[derive(Clone, Copy, Debug)]
pub struct ItemSlot {
    pub id: ActivityId,
    pub bounds: Rect,
}

#[derive(Clone, Debug)]
pub enum ChromeSlot {
    AppIcon,
    ArrowUp,
    ArrowDown,
    Settings,
    ResizeHandle,
}

#[derive(Clone, Debug)]
pub struct ActivityLayout {
    pub items: Vec<ItemSlot>,
    pub resize_handle: Option<Rect>,
    pub chrome: Vec<(ChromeSlot, Rect)>,
}
