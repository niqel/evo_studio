use std::num::NonZeroUsize;
use std::sync::Arc;
use crate::text::TextEngine;

mod model; // Model, Msg, update, apply_action
use model::{Model, Msg, update, apply_action};

// Usamos la ActivityBar directamente (catálogo/params)
use crate::scene::acetates::ActivityBar;
// Sketcher (dominio → codominio Scene + Texts)
use crate::sketcher::{Sketcher, SketchInput};

use winit::event::{ElementState, KeyEvent, MouseButton, WindowEvent};
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes},
};

use vello::{
    peniko::Color,
    util::{RenderContext, RenderSurface},
    wgpu, AaConfig, RenderParams, Renderer, RendererOptions,
};
use vello::wgpu::PresentMode;
use vello::wgpu::util::TextureBlitter;

/* ----------------------- Helpers WGPU ----------------------- */

fn make_target_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("vello-target"),
        size: wgpu::Extent3d {
            width: width.max(1),
            height: height.max(1),
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    })
}

/* ----------------------- App (Vello + Winit) ----------------------- */

pub struct App {
    // Ventana
    window: Option<Arc<Window>>,
    window_id: Option<winit::window::WindowId>,

    // Vello / WGPU
    rc: RenderContext,
    surface: Option<RenderSurface<'static>>,
    device_idx: Option<usize>,
    renderer: Option<Renderer>,
    blitter: Option<TextureBlitter>,
    target_tex: Option<wgpu::Texture>,

    // Motor de texto (opcional; se inicializa en resumed)
    text: Option<TextEngine>,

    // Modelo puro (MVI)
    model: Model,

    // Catálogo/params estáticos de la ActivityBar
    activity_bar: ActivityBar,

    // Sketcher (dominio -> Scene + Texts)
    sketcher: Sketcher,

    // Teclado / Mouse
    modifiers: ModifiersState,
    last_cursor: (f64, f64),
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            window_id: None,
            rc: RenderContext::new(),
            surface: None,
            device_idx: None,
            renderer: None,
            blitter: None,
            target_tex: None,
            text: None, // ← se construye en resumed()
            model: Model::new(),
            activity_bar: ActivityBar::new(),
            sketcher: Sketcher::new(),
            modifiers: ModifiersState::empty(),
            last_cursor: (0.0, 0.0),
        }
    }

    fn window_size(&self) -> PhysicalSize<u32> {
        self.window
            .as_deref()
            .map(Window::inner_size)
            .unwrap_or(PhysicalSize::new(1, 1))
    }

    fn draw(&mut self) {
        if let (Some(surface), Some(idx)) = (self.surface.as_mut(), self.device_idx) {
            // Asegurar render target
            self.target_tex.get_or_insert_with(|| {
                make_target_texture(
                    &self.rc.devices[idx].device,
                    surface.config.width,
                    surface.config.height,
                )
            });

            // 1) Dominio validado para el Sketcher (desde el Model + ActivityBar)
            let input = SketchInput::new(
                surface.config.width,
                surface.config.height,
                &self.model.metrics,
                &self.model.toggles,
                &self.activity_bar,             // &ActivityBar directa
                self.model.activity_hovered,     // Option<ActivityId>
                self.model.activity_active,      // Option<ActivityId>
            )
            .expect("SketchInput válido");

            // 2) Dominio → salida (Scene + Texts) de forma pura
            let out = self.sketcher.draw(&input);

            // 3) Render Vello a textura
            if let Some(r) = self.renderer.as_mut() {
                r.render_to_texture(
                    &self.rc.devices[idx].device,
                    &self.rc.devices[idx].queue,
                    &out.scene,
                    &self
                        .target_tex
                        .as_ref()
                        .unwrap()
                        .create_view(&wgpu::TextureViewDescriptor::default()),
                    &RenderParams {
                        base_color: Color::from_rgba8(0, 0, 0, 255),
                        width: surface.config.width,
                        height: surface.config.height,
                        antialiasing_method: AaConfig::Area,
                    },
                )
                .expect("render_to_texture");
            }

            // 4) Frame + blit + texto en el MISMO encoder
            if let Ok(frame) = surface.surface.get_current_texture() {
                let dst = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let src = self
                    .target_tex
                    .as_ref()
                    .unwrap()
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut enc = self.rc.devices[idx]
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("blit+text") });

                // Blit Vello → Surface
                if let Some(blit) = self.blitter.as_ref() {
                    blit.copy(&self.rc.devices[idx].device, &mut enc, &src, &dst);
                }

                // Texto encima (si hay motor de texto)
                if let Some(text) = self.text.as_mut() {
                    text.render(
                        &self.rc.devices[idx].device,
                        &self.rc.devices[idx].queue,
                        &mut enc,
                        &dst,
                        surface.config.width,
                        surface.config.height,
                        out.texts(), // comandos de texto del Sketcher
                    );
                }

                self.rc.devices[idx].queue.submit(std::iter::once(enc.finish()));
                frame.present();
            }
        }
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if let (Some(surface), Some(idx)) = (self.surface.as_mut(), self.device_idx) {
            self.rc
                .resize_surface(surface, new_size.width.max(1), new_size.height.max(1));
            self.target_tex = Some(make_target_texture(
                &self.rc.devices[idx].device,
                new_size.width.max(1),
                new_size.height.max(1),
            ));
        }
    }

    #[inline]
    fn request_redraw(&self) {
        if let Some(w) = self.window.as_deref() {
            w.request_redraw();
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // ventana
        self.window = Some(Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("evo_studio (Vello)")
                        .with_inner_size(PhysicalSize::new(1200, 800)),
                )
                .expect("window"),
        ));

        // id
        self.window_id = self.window.as_deref().map(Window::id);

        // surface
        self.surface = self.window.as_ref().map(|w| {
            pollster::block_on(self.rc.create_surface(
                w.clone(),
                w.inner_size().width,
                w.inner_size().height,
                PresentMode::AutoVsync,
            ))
            .expect("create_surface")
        });

        // device
        self.device_idx = self
            .surface
            .as_ref()
            .map(|s| pollster::block_on(self.rc.device(Some(&s.surface))).expect("device_idx"));

        // renderer
        self.renderer = self.device_idx.map(|idx| {
            Renderer::new(
                &self.rc.devices[idx].device,
                RendererOptions {
                    use_cpu: false,
                    antialiasing_support: vello::AaSupport::all(),
                    num_init_threads: Some(NonZeroUsize::new(1).unwrap()),
                    pipeline_cache: None,
                },
            )
            .expect("renderer")
        });

        // blitter
        self.blitter = self
            .device_idx
            .zip(self.surface.as_ref())
            .map(|(idx, s)| TextureBlitter::new(&self.rc.devices[idx].device, s.config.format));

        // target inicial
        self.target_tex = self
            .device_idx
            .zip(self.surface.as_ref())
            .map(|(idx, s)| {
                make_target_texture(
                    &self.rc.devices[idx].device,
                    s.config.width,
                    s.config.height,
                )
            });

        // text engine (opcional)
        self.text = self
            .device_idx
            .zip(self.surface.as_ref())
            .map(|(idx, s)| TextEngine::new(s.config.format, &self.rc.devices[idx].device, &self.rc.devices[idx].queue));

        self.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        win_id: winit::window::WindowId,
        ev: WindowEvent,
    ) {
        if self.window_id.is_some_and(|id| id == win_id) {
            match ev {
                WindowEvent::CloseRequested => event_loop.exit(),

                WindowEvent::Resized(size) => {
                    self.resize(size);
                    self.request_redraw();
                }

                WindowEvent::RedrawRequested => {
                    self.draw();
                }

                WindowEvent::ModifiersChanged(m) => {
                    self.modifiers = m.state();
                },

                // Tecla presionada (Ctrl+B / Ctrl+J) → Msg y update puro
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key,
                            ..
                        },
                    ..
                } => {
                    let msg = match (physical_key, self.modifiers.control_key()) {
                        (PhysicalKey::Code(KeyCode::KeyB), true) => Some(Msg::ToggleSidebar),
                        (PhysicalKey::Code(KeyCode::KeyJ), true) => Some(Msg::TogglePanel),
                        _ => None,
                    };

                    if let Some(msg) = msg {
                        let (next, act) =
                            update(std::mem::take(&mut self.model), msg, &self.activity_bar);
                        self.model = next;
                        if let Some(a) = act { apply_action(&mut self.model, &a); }
                        self.request_redraw();
                    }
                }

                // Movimiento del cursor → hover ActivityBar (hit-testing puro)
                WindowEvent::CursorMoved { position, .. } => {
                    self.last_cursor = (position.x, position.y);
                    let size = self.window_size();

                    let (next, _act) = update(
                        std::mem::take(&mut self.model),
                        Msg::CursorMoved {
                            x: position.x,
                            y: position.y,
                            win_w: size.width,
                            win_h: size.height,
                        },
                        &self.activity_bar,
                    );
                    self.model = next;

                    self.request_redraw();
                }

                // Mouse down → ActivityBar (hit & acción pura)
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button: MouseButton::Left,
                    ..
                } => {
                    let size = self.window_size();
                    let (x, y) = self.last_cursor;
                    let (next, act) = update(
                        std::mem::take(&mut self.model),
                        Msg::MouseDown {
                            x, y, win_w: size.width, win_h: size.height,
                        },
                        &self.activity_bar,
                    );
                    self.model = next;
                    if let Some(a) = act { apply_action(&mut self.model, &a); }
                    self.request_redraw();
                }

                // Mouse up → termina estado activo
                WindowEvent::MouseInput {
                    state: ElementState::Released,
                    button: MouseButton::Left,
                    ..
                } => {
                    let (next, _act) =
                        update(std::mem::take(&mut self.model), Msg::MouseUp, &self.activity_bar);
                    self.model = next;
                    self.request_redraw();
                }

                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, _el: &ActiveEventLoop) {
        // Redibuja si estamos vivos (idle draws suaves)
        self.request_redraw();
    }
}
