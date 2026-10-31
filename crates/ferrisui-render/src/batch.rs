use crate::primitives::RectInstance;

#[derive(Default)]
pub struct RenderBatch {
    pub rects: Vec<RectInstance>,
}

impl RenderBatch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_rect(&mut self, instance: RectInstance) {
        self.rects.push(instance);
    }

    pub fn clear(&mut self) {
        self.rects.clear();
    }
}
