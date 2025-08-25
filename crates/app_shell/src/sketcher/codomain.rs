use vello::Scene;
use super::display::TextCommand;

pub struct SketchOutput {
    pub scene: Scene,
    pub texts: Vec<TextCommand>,
}

impl SketchOutput {
    pub fn into_scene(self) -> Scene { self.scene }
    pub fn texts(&self) -> &[TextCommand] { &self.texts }
}
