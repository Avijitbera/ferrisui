use std::{fmt::write, sync::Arc};

#[derive(Clone)]
pub struct Callback<Arg = ()> {
    func: Arc<dyn Fn(Arg) + Send + Sync>,
}

impl<Arg> Callback<Arg> {
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(Arg) + Send + Sync + 'static,
    {
        Self {
            func: Arc::new(func),
        }
    }
    pub fn emit(&self, arg: Arg) {
        (self.func)(arg);
    }
}

impl<Arg> std::fmt::Debug for Callback<Arg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Callback(...)")
    }
}
