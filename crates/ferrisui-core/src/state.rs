use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Clone)]
pub struct Signal<T> {
    value: Arc<RwLock<T>>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Signal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Signal").field(&*self.value.read()).finish()
    }
}

impl<T: Clone> Signal<T> {
    pub fn new(initial: T) -> Self {
        Self {
            value: Arc::new(RwLock::new(initial)),
        }
    }

    /// Read the current value of the Signal.
    pub fn get(&self) -> T {
        self.value.read().clone()
    }
    /// Replace the value and notify listeners.
    pub fn set(&self, new_val: T) {
        *self.value.write() = new_val;
    }
    /// Mutate the value in-place using a closure.
    pub fn update<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        f(&mut *self.value.write())
    }
}
