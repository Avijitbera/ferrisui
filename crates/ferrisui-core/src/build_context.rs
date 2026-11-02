use crate::state::Signal;
use std::any::Any;

pub struct BuildContext {
    states: Vec<Box<dyn Any>>,
    state_index: usize,
}

impl BuildContext {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            state_index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.state_index = 0;
    }

    pub fn state<T: Clone + 'static>(&mut self, initial: T) -> Signal<T> {
        if self.state_index < self.states.len() {
            if let Some(sig) = self.states[self.state_index].downcast_ref::<Signal<T>>() {
                let s = sig.clone();
                self.state_index += 1;
                return s;
            }
        }
        let sig = Signal::new(initial);
        self.states.push(Box::new(sig.clone()));
        self.state_index += 1;
        sig
    }

    pub fn use_signal<T: Clone + 'static>(&mut self, initial: T) -> Signal<T> {
        self.state(initial)
    }
}

impl Default for BuildContext {
    fn default() -> Self {
        Self::new()
    }
}

