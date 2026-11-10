use crate::state::Signal;
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Declarative build context provided during the widget tree construction phase.
///
/// Holds reactive local states (`Signal<T>`) and ambient environment providers
/// (such as design themes, active locale, window metrics, etc.).
pub struct BuildContext {
    states: Vec<Box<dyn Any>>,
    state_index: usize,
    environment: HashMap<TypeId, Box<dyn Any>>,
}

impl BuildContext {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            state_index: 0,
            environment: HashMap::new(),
        }
    }

    /// Resets the internal state allocator index before each rebuild pass.
    pub fn reset(&mut self) {
        self.state_index = 0;
    }

    /// Allocates or retrieves a persistent reactive state signal across view builds.
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

    /// Hook-style alias for `state(initial)`.
    pub fn use_signal<T: Clone + 'static>(&mut self, initial: T) -> Signal<T> {
        self.state(initial)
    }

    /// Inserts or overrides an ambient environment value in the context.
    pub fn provide<T: 'static>(&mut self, value: T) {
        self.environment.insert(TypeId::of::<T>(), Box::new(value));
    }

    /// Retrieves an immutable reference to an ambient environment value of type `T`.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.environment
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    /// Retrieves a mutable reference to an ambient environment value of type `T`.
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.environment
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut::<T>())
    }

    /// Checks if an ambient environment value of type `T` exists in the context.
    pub fn has<T: 'static>(&self) -> bool {
        self.environment.contains_key(&TypeId::of::<T>())
    }
}

impl Default for BuildContext {
    fn default() -> Self {
        Self::new()
    }
}
