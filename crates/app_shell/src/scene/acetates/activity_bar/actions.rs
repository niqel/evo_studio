//! actions.rs — Tipos puros (ID y Action) sin dependencias de vello.

/// Identificador de un ítem de la ActivityBar.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ActivityId(pub u32);

/// Acciones que puede disparar un ítem de la ActivityBar.
#[derive(Clone, Debug)]
pub enum ActivityAction {
    ShowExplorer,
    ShowSearch,
    ShowSourceControl,
    ShowRun,
    ShowExtensions,
    ShowSettings,
    TogglePanel,
    Custom(&'static str),
}

