use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::sync::Arc;
use wgpu::*;

pub struct GpuContext {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
}

impl GpuContext {
    pub async fn new<W>(
        window: Arc<W>,
        width: u32,
        height: u32,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        W: HasWindowHandle + HasDisplayHandle + Send + Sync + 'static,
    {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            flags: InstanceFlags::default(),
            dx12_shader_compiler: Dx12Compiler::default(),
            gles_minor_version: Gles3MinorVersion::default(),
        });

        let surface = instance.create_surface(window)?;

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or("Failed to find suitable GPU adapter")?;

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("FerrisUI Device"),
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                    memory_hints: MemoryHints::Performance,
                },
                None,
            )
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            height: height.max(1),
            width: width.max(1),
            present_mode: PresentMode::AutoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
            surface,
            surface_config,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }
}
