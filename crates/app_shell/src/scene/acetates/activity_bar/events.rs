//! events.rs — Resultados de mouse down (puro). Sin estado ni handlers mutables.

use super::actions::ActivityId;

#[derive(Clone, Copy, Debug)]
pub enum MouseDownResult {
    Item(ActivityId),
    StartResize,
    None,
}
