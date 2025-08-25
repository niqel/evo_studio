// scene/mod.rs

pub mod colors;        // colores globales
pub mod metrics;       // tamaños (DIP)
pub mod state;         // estado/toggles globales (sidebar/panel)
pub mod layout;        // layout global del shell
pub mod acetates;      // capas/acetatos (title bar, activity bar, etc.)
pub mod acetate;       // <— el trait Acetate vivía aquí

// Re-exports convenientes
pub use state::UiToggles;
pub use metrics::UiMetrics;
