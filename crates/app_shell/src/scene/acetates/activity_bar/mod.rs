pub mod actions;
pub mod component;
pub mod layout;
pub mod view;
pub mod hit;
mod events;

// Reexports públicos
pub use actions::{ActivityAction, ActivityId};
pub use component::ActivityBar;

// Estos dos ahora se reexportan desde component.rs (no desde layout.rs)
pub use component::{ActivityLayout, ItemSlot};

pub use events::MouseDownResult;
