// C:\repos\evo_studio\crates\app_shell\src\sketcher\domain.rs
use crate::scene::{metrics::UiMetrics, state::UiToggles, acetates::ActivityBar};
use crate::scene::acetates::activity_bar::ActivityId;

#[derive(Debug)]
pub enum SketchInputError { ZeroWidth, ZeroHeight }

pub struct SketchInput<'a> {
    pub width: u32,
    pub height: u32,
    pub metrics: &'a UiMetrics,
    pub toggles: &'a UiToggles,
    pub activity: &'a ActivityBar,
    pub activity_hovered: Option<ActivityId>,
    pub activity_active:  Option<ActivityId>,
}

impl<'a> SketchInput<'a> {
    pub fn new(
        width: u32,
        height: u32,
        metrics: &'a UiMetrics,
        toggles: &'a UiToggles,
        activity: &'a ActivityBar,
        activity_hovered: Option<ActivityId>,
        activity_active:  Option<ActivityId>,
    ) -> Result<Self, SketchInputError> {
        if width == 0 { return Err(SketchInputError::ZeroWidth); }
        if height == 0 { return Err(SketchInputError::ZeroHeight); }
        Ok(Self {
            width, height, metrics, toggles,
            activity, activity_hovered, activity_active,
        })
    }
}
