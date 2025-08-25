//! item/component.rs — Definición del ActivityItem (pieza independiente).

use super::super::actions::{ActivityAction, ActivityId};

pub struct ActivityItem {
    pub id: ActivityId,
    pub action: ActivityAction,
    pub tooltip: Option<String>,
    pub disabled: bool,
}

impl ActivityItem {
    pub fn new(id: u32, action: ActivityAction) -> Self {
        Self {
            id: ActivityId(id),
            action,
            tooltip: None,
            disabled: false,
        }
    }

    #[inline] pub fn id(&self) -> ActivityId { self.id }
    #[inline] pub fn action(&self) -> &ActivityAction { &self.action }
}
