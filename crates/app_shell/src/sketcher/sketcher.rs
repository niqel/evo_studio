use vello::kurbo::Rect;
use crate::scene::{self, colors};
use super::{DrawOp, emit_scene_and_texts};
use super::domain::SketchInput;
use super::codomain::SketchOutput;
use crate::scene::acetates as ui;

pub struct Sketcher;

#[inline]
fn hline(y: f64, x0: f64, x1: f64) -> Rect { Rect::new(x0, y.max(0.0), x1, (y + 1.0).max(y)) }
#[inline]
fn vline(x: f64, y0: f64, y1: f64) -> Rect { Rect::new(x.max(0.0), y0, (x + 1.0).max(x), y1) }

impl Sketcher {
    pub fn new() -> Self { Self }

    #[inline]
    fn with_layout<T>(input: &SketchInput<'_>, f: impl FnOnce(scene::layout::UiLayout) -> T) -> T {
        f(scene::layout::compute_layout(input.width, input.height, input.metrics, input.toggles))
    }

    pub fn draw(&self, input: &SketchInput<'_>) -> SketchOutput {
        Self::with_layout(input, |ui| {
            let emitted = emit_scene_and_texts(
                std::iter::once(DrawOp::FillRect {
                    rect: Rect::new(0.0, 0.0, input.width as f64, input.height as f64),
                    color: colors::BG,
                })
                .chain(ui::TitleBar::view(ui.title))
                .chain(std::iter::once(DrawOp::FillRect {
                    rect: hline(ui.title.y1 - 1.0, 0.0, input.width as f64),
                    color: colors::SEPARATOR,
                }))
                .chain(
                    input.activity
                        .view_with(ui.activity, input.activity_hovered, input.activity_active)
                )
                .chain(std::iter::once(DrawOp::FillRect {
                    rect: vline(ui.activity.x1 - 1.0, ui.activity.y0, ui.activity.y1),
                    color: colors::SEPARATOR,
                }))
                .chain(
                    input.toggles.sidebar()
                        .then(|| {
                            ui::SideBar::view(ui.sidebar)
                                .into_iter()
                                .chain(std::iter::once(DrawOp::FillRect {
                                    rect: vline(ui.sidebar.x1 - 1.0, ui.sidebar.y0, ui.sidebar.y1),
                                    color: colors::SEPARATOR,
                                }))
                                .collect::<Vec<_>>()
                        })
                        .into_iter()
                        .flatten()
                )
                .chain(ui::EditorArea::view(ui.editor))
                .chain(
                    input.toggles.panel()
                        .then(|| {
                            ui::Panel::view(ui.panel)
                                .into_iter()
                                .chain(std::iter::once(DrawOp::FillRect {
                                    rect: hline(ui.panel.y0, ui.panel.x0, ui.panel.x1),
                                    color: colors::SEPARATOR,
                                }))
                                .collect::<Vec<_>>()
                        })
                        .into_iter()
                        .flatten()
                )
                .chain(ui::StatusBar::view(ui.status)),
                input.width,
                input.height,
            );

            SketchOutput { scene: emitted.scene, texts: emitted.texts }
        })
    }
}
