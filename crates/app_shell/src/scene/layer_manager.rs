use vello::Scene;
use vello::kurbo::{Rect, Affine};
use vello::peniko::Fill;

use super::colors;
use super::metrics::UiMetrics;
use super::state::UiToggles;
use super::acetates::{TitleBar, ActivityBar, SideBar, EditorArea, Panel, StatusBar};
use super::acetate::Acetate;

// ActivityBar: acciones y resultados de eventos
use super::acetates::activity_bar::{ActivityAction, MouseDownResult};
use super::layout;

pub struct LayerManager {
    pub metrics: UiMetrics,   // DIP (px lógicos)
    pub toggles: UiToggles,   // estado global (sidebar/panel visibles)
    // Capas
    title: TitleBar,
    activity: ActivityBar,
    sidebar: SideBar,
    editor: EditorArea,
    panel: Panel,
    status: StatusBar,
}

// Calcula el layout una sola vez y lo pasa al closure.
// Toma referencias para no mover metrics/toggles.
fn with_layout<T>(
    width: u32,
    height: u32,
    metrics: &UiMetrics,
    toggles: &UiToggles,
    f: impl FnOnce(super::layout::UiLayout) -> T,
) -> T {
    f(super::layout::compute_layout(width, height, metrics, toggles))
}

impl LayerManager {
    pub fn new() -> Self {
        Self {
            metrics: UiMetrics::new(),
            toggles: UiToggles::default_on(),
            title: TitleBar::new(),
            activity: ActivityBar::new(),
            sidebar: SideBar::new(),
            editor: EditorArea::new(),
            panel: Panel::new(),
            status: StatusBar::new(),
        }
    }

    /// Acceso de solo lectura a la ActivityBar (útil para el Sketcher/visteo puro).
    #[inline]
    pub fn activity_bar(&self) -> &ActivityBar {
        &self.activity
    }

    // Helpers para toggles (API limpia)
    pub fn toggle_sidebar(&mut self) { self.toggles.toggle_sidebar(); }
    pub fn toggle_panel(&mut self)   { self.toggles.toggle_panel();  }

    pub fn draw_all(&mut self, scene: &mut Scene, width: u32, height: u32) {
        // Fondo
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            colors::BG,
            None,
            &Rect::new(0.0, 0.0, width as f64, height as f64),
        );

        // Una sola llamada a compute_layout (vía with_layout)
        with_layout(width, height, &self.metrics, &self.toggles, |l| {
            self.title.draw(scene, l.title);
            self.activity.draw(scene, l.activity);
            if self.toggles.sidebar() { self.sidebar.draw(scene, l.sidebar); }
            self.editor.draw(scene, l.editor);
            if self.toggles.panel()   { self.panel.draw(scene, l.panel); }
            self.status.draw(scene, l.status);
        });
    }

    /* =========================
       Integración de mouse
       ========================= */

    /// Cursor movido → delega hover a ActivityBar.
    pub fn on_cursor_moved(&mut self, px: f64, py: f64, width: u32, height: u32) {
        let layout_now = layout::compute_layout(width, height, &self.metrics, &self.toggles);
        // El propio ActivityBar actualiza su UI interna (hover enter/leave) y devuelve el Hit
        let _hit = self.activity.on_cursor_moved(layout_now.activity, (px, py));
    }

    /// Mouse presionado → selecciona item / inicia resize.
    pub fn on_mouse_down(&mut self, px: f64, py: f64, width: u32, height: u32) {
        let layout_now = layout::compute_layout(width, height, &self.metrics, &self.toggles);
        match self.activity.on_mouse_down(layout_now.activity, (px, py)) {
            MouseDownResult::Item(id) => {
                // IMPORTANTE: clonar para terminar el préstamo inmutable antes de mutar self
                if let Some(action) = self.activity.action_of(id).cloned() {
                    self.apply_action(&action);
                }
            }
            MouseDownResult::StartResize | MouseDownResult::None => {}
        }
    }

    /// Mouse liberado → termina resize si estaba activo.
    pub fn on_mouse_up(&mut self) {
        let _ = self.activity.on_mouse_up();
    }

    /// Aplica las acciones provenientes de la ActivityBar.
    fn apply_action(&mut self, action: &ActivityAction) {
        match action {
            ActivityAction::TogglePanel => {
                self.toggle_panel();
            }
            ActivityAction::ShowExplorer | ActivityAction::ShowSearch => {
                // Por ahora, asegura Sidebar visible (más adelante se elegirá la vista interna).
                self.toggles.set_sidebar(true);
            }
            // Otras acciones quedan como no-op por ahora; las iremos mapeando.
            _ => {}
        }
    }
}
