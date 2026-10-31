pub mod batch;
pub mod clip;
pub mod context;
pub mod pipline;
pub mod primitives;
pub mod renderer;
pub use context::GpuContext;
pub use primitives::{RectInstance, WindowUniforms};
pub use renderer::Renderer;

pub mod prelude {
    pub use crate::context::GpuContext;
    pub use crate::primitives::RectInstance;
    pub use crate::renderer::Renderer;
}
