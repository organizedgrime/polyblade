use crate::prelude::WindowScene;
use three_d::FrameInput;

pub trait Renderable {
    fn render(&self, scene: &mut WindowScene, frame_input: &FrameInput);
}
