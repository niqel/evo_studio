// crates/app_shell/src/runner/runner.rs
//
// Runner funcional:
// - Traduce eventos winit → inputs puros del Renderer
// - Renderer gobierna Snapshot (sin que Runner toque campos privados)
// - Redraw on demand
//
// Maneja: resize, cursor move, mouse L down/up, teclado (sin auto-repeat)

use std::collections::HashSet;

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    error::EventLoopError,
    event::{ElementState, KeyEvent, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, KeyCode, ModifiersState, NamedKey, PhysicalKey},
    window::{WindowAttributes, WindowId},
};

use crate::renderer::{
    KeyInput, KeyModifiers, LogicalKey, Renderer, Snapshot,
};

pub struct Runner {
    window: Option<winit::window::Window>,
    renderer: Renderer,
    snapshot: Option<Snapshot>,
    needs_redraw: bool,
    last_cursor: (f64, f64),

    // teclado
    pressed_codes: HashSet<KeyCode>, // para filtrar auto-repeat
    modifiers: ModifiersState,       // último estado de modificadores
}

impl Runner {
    fn new() -> Self {
        Self {
            window: None,
            renderer: Renderer::new(),
            snapshot: None,
            needs_redraw: false,
            last_cursor: (0.0, 0.0),

            pressed_codes: HashSet::new(),
            modifiers: ModifiersState::empty(),
        }
    }

    /// Punto de entrada del bucle
    pub fn run() -> Result<(), EventLoopError> {
        EventLoop::new()?.run_app(Box::leak(Box::new(Runner::new())))
    }

    #[inline]
    fn request_redraw(&mut self) {
        if let Some(w) = self.window.as_ref() {
            w.request_redraw();
        }
    }

    // ----------- Helpers de mapping teclado (winit → tipos puros) -----------

    fn map_logical_key(logical: Key) -> LogicalKey {
        match logical {
            Key::Character(s) => s.chars().next().map(LogicalKey::Char).unwrap_or(LogicalKey::None),
            Key::Named(NamedKey::Escape) => LogicalKey::Escape,
            Key::Named(NamedKey::Enter) => LogicalKey::Enter,
            Key::Named(NamedKey::Tab) => LogicalKey::Tab,
            Key::Named(NamedKey::Backspace) => LogicalKey::Backspace,
            Key::Named(NamedKey::Space) => LogicalKey::Space,
            Key::Named(other) => LogicalKey::Named(format!("{:?}", other)),
            _ => LogicalKey::None,
        }
    }

    fn current_mods(&self) -> KeyModifiers {
        KeyModifiers {
            ctrl: self.modifiers.control_key(),
            alt: self.modifiers.alt_key(),
            shift: self.modifiers.shift_key(),
            logo: self.modifiers.super_key(),
        }
    }
}

impl ApplicationHandler for Runner {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.window.is_none() {
            let win = el
                .create_window(
                    WindowAttributes::default()
                        .with_title("evo_studio")
                        .with_inner_size(PhysicalSize::new(1200, 800)),
                )
                .expect("no se pudo crear la ventana");

            let size = win.inner_size();
            let snap = self
                .renderer
                .snapshot_initial(size.width, size.height, (0, 0, 255, 255)); // azul

            self.window = Some(win);
            self.snapshot = Some(snap);
            self.needs_redraw = true; // primer frame
        }
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => el.exit(),

            WindowEvent::Resized(new_size) => {
                if let Some(s) = self.snapshot.take() {
                    self.snapshot = Some(self.renderer.snapshot_resize(
                        s,
                        new_size.width,
                        new_size.height,
                    ));
                }
                self.needs_redraw = true;
            }

            WindowEvent::ScaleFactorChanged { .. } => {
                if let Some(win) = self.window.as_ref() {
                    let sz = win.inner_size();
                    if let Some(s) = self.snapshot.take() {
                        self.snapshot =
                            Some(self.renderer.snapshot_resize(s, sz.width, sz.height));
                    }
                    self.needs_redraw = true;
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                self.last_cursor = (position.x, position.y);
                if let Some(s) = self.snapshot.take() {
                    self.snapshot =
                        Some(Renderer::on_cursor_moved(s, position.x, position.y));
                }
                self.needs_redraw = true;
            }

            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                if let Some(s) = self.snapshot.take() {
                    self.snapshot = Some(Renderer::on_mouse_down_left(
                        s,
                        self.last_cursor.0,
                        self.last_cursor.1,
                    ));
                }
                self.needs_redraw = true;
            }

            WindowEvent::MouseInput {
                state: ElementState::Released,
                button: MouseButton::Left,
                ..
            } => {
                if let Some(s) = self.snapshot.take() {
                    self.snapshot = Some(Renderer::on_mouse_up_left(
                        s,
                        self.last_cursor.0,
                        self.last_cursor.1,
                    ));
                }
                self.needs_redraw = true;
            }

            WindowEvent::ModifiersChanged(m) => {
                self.modifiers = m.state();
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        physical_key,
                        logical_key,
                        repeat,
                        ..
                    },
                ..
            } => {
                // Cerrar app con Escape (efecto colateral del OS → Runner)
                if matches!(logical_key, Key::Named(NamedKey::Escape))
                    && matches!(state, ElementState::Pressed)
                {
                    el.exit();
                    return;
                }

                // Filtrado de auto-repeat
                match (state, &physical_key) {
                    (ElementState::Pressed, PhysicalKey::Code(code)) => {
                        if repeat || !self.pressed_codes.insert(*code) {
                            // auto-repeat → ignoramos
                            return;
                        }
                    }
                    (ElementState::Released, PhysicalKey::Code(code)) => {
                        self.pressed_codes.remove(code);
                    }
                    _ => {}
                }

                // Mapear a input puro
                let input = KeyInput {
                    is_pressed: matches!(state, ElementState::Pressed),
                    logical: Self::map_logical_key(logical_key),
                    modifiers: self.current_mods(),
                };

                // Delegar al renderer (puro)
                if let Some(s) = self.snapshot.take() {
                    self.snapshot = Some(Renderer::on_key_input(s, input));
                }
                self.needs_redraw = true;
            }

            WindowEvent::RedrawRequested => {
                if let Some(snap) = self.snapshot.take() {
                    let next = self.renderer.render(snap);
                    self.snapshot = Some(next);

                    if self
                        .snapshot
                        .as_ref()
                        .is_some_and(|s| s.has_active_animations())
                    {
                        self.request_redraw();
                    }
                }
            }

            _ => {}
        }

        if self.needs_redraw {
            self.needs_redraw = false;
            self.request_redraw();
        }
    }
}
