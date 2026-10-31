use ferrisui_core::geometry::Rect;

pub struct ClipStack {
    stack: Vec<Rect>,
}

impl ClipStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, rect: Rect) {
        if let Some(current) = self.stack.last() {
            let x1 = current.origin.x.max(rect.origin.x);
            let y1 = current.origin.y.max(rect.origin.y);
            let x2 = (current.origin.x + current.size.width).min(rect.origin.x + rect.size.width);
            let y2 = (current.origin.y + current.size.height).min(rect.origin.y + rect.size.height);

            let intersected = Rect::new(x1, y1, (x2 - x1).max(0.0), (y2 - y1).max(0.0));
            self.stack.push(intersected);
        } else {
            self.stack.push(rect);
        }
    }

    pub fn pop(&mut self) -> Option<Rect> {
        self.stack.pop()
    }

    pub fn current(&self) -> Option<Rect> {
        self.stack.last().copied()
    }
}
