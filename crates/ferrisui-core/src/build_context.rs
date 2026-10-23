use crate::state::Signal;

pub struct BuildContext {}

impl BuildContext {
    pub fn new() -> Self {
        Self {}
    }

    pub fn use_signal<T: Clone + 'static>(&mut self, initial: T) -> Signal<T> {
        Signal::new(initial)
    }
}

impl Default for BuildContext {
    fn default() -> Self {
        Self::new()
    }
}
