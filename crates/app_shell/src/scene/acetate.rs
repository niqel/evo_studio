use vello::{Scene};
use vello::kurbo::Rect;

pub trait Acetate {
    fn draw(&mut self, scene: &mut Scene, rect: Rect);
}
