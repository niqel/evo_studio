#[derive(Debug)] 
pub struct UiMetrics {
    // px lógicos (DIP). Convierte a físicos con scale_factor cuando haga falta.
    titlebar_height: f64,
    activitybar_width: f64,
    statusbar_height: f64,
    panel_height: f64,
    sidebar_width: f64,
}

impl UiMetrics {
    /// Métricas por defecto (valores razonables).
    pub fn new() -> Self { Self::default() }

    /// (Opcional) Construye con valores explícitos.
    pub fn from_values(
        titlebar_height: f64,
        activitybar_width: f64,
        statusbar_height: f64,
        panel_height: f64,
        sidebar_width: f64,
    ) -> Self {
        Self { titlebar_height, activitybar_width, statusbar_height, panel_height, sidebar_width }
    }

    // Getters solo lectura (DIP)
    pub fn titlebar_height(&self)   -> f64 { self.titlebar_height }
    pub fn activitybar_width(&self) -> f64 { self.activitybar_width }
    pub fn statusbar_height(&self)  -> f64 { self.statusbar_height }
    pub fn panel_height(&self)      -> f64 { self.panel_height }
    pub fn sidebar_width(&self)     -> f64 { self.sidebar_width }
}

impl Default for UiMetrics {
    fn default() -> Self {
        Self {
            titlebar_height:   36.0,
            activitybar_width: 48.0,
            statusbar_height:  22.0,
            panel_height:      160.0,
            sidebar_width:     280.0,
        }
    }
}
