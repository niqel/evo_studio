use crate::scene::{
    metrics::UiMetrics,
    state::UiToggles,
    layout,
    acetates::{
        ActivityBar,
        activity_bar::{self, ActivityId, ActivityAction, MouseDownResult}, // ← aquí
    },
};

#[derive(Debug)]
pub struct Model {
    pub metrics: UiMetrics,
    pub toggles: UiToggles,
    pub activity_hovered: Option<ActivityId>,
    pub activity_active:  Option<ActivityId>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            metrics: UiMetrics::new(),
            toggles: UiToggles::default_on(),
            activity_hovered: None,
            activity_active:  None,
        }
    }
}

impl Default for Model {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, Copy)]
pub enum Msg {
    CursorMoved { x: f64, y: f64, win_w: u32, win_h: u32 },
    MouseDown   { x: f64, y: f64, win_w: u32, win_h: u32 },
    MouseUp,
    ToggleSidebar,
    TogglePanel,
}

/// Reducer “puro”: devuelve el nuevo Model + posible Action.
pub fn update(mut m: Model, msg: Msg, activity: &ActivityBar) -> (Model, Option<ActivityAction>) {
    use Msg::*;
    let mut action = None;

    match msg {
        CursorMoved { x, y, win_w, win_h } => {
            let ui = layout::compute_layout(win_w, win_h, &m.metrics, &m.toggles);
            m.activity_hovered = activity_bar::hit::hovered(activity, ui.activity, (x, y));
        }
        MouseDown { x, y, win_w, win_h } => {
            let ui = layout::compute_layout(win_w, win_h, &m.metrics, &m.toggles);
            match activity_bar::hit::mouse_down(activity, ui.activity, (x, y)) {
                MouseDownResult::Item(id) => {
                    m.activity_active = Some(id);
                    action = activity.action_of(id).cloned();
                }
                MouseDownResult::StartResize => { /* TODO: resize */ }
                MouseDownResult::None => {}
            }
        }
        MouseUp => {
            m.activity_active = None;
        }
        ToggleSidebar => m.toggles.toggle_sidebar(),
        TogglePanel   => m.toggles.toggle_panel(),
    }

    (m, action)
}

/// Aplica efectos secundarios de una `ActivityAction` sobre el `Model`.
pub fn apply_action(m: &mut Model, action: &ActivityAction) {
    match action {
        ActivityAction::TogglePanel => m.toggles.toggle_panel(),
        ActivityAction::ShowExplorer | ActivityAction::ShowSearch => m.toggles.set_sidebar(true),
        _ => {}
    }
}
