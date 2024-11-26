use std::default::Default;
use std::iter;
use std::num::NonZeroU32;
use std::sync::Arc;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;
use wgpu::{include_wgsl, Backends, CommandEncoderDescriptor, DeviceDescriptor, Instance, InstanceDescriptor, LoadOp, Operations, PipelineLayoutDescriptor, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, StoreOp, SurfaceConfiguration, TextureUsages, TextureViewDescriptor};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};
#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::platform::windows::WindowExtWindows;
use winit::window::WindowId;
use twilight_wgpu_backend::Backend;

struct State<'a> {
    window: Arc<winit::window::Window>,
    backend: Backend<'a>
}

struct Application<'a> {
    state: Option<State<'a>>,
    // #[cfg(target_arch = "wasm32")]
    // canvas_element: String
}

impl<'a> ApplicationHandler<()> for Application<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(Default::default()).unwrap());
        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            // backends: self.backends,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapters = instance.enumerate_adapters(Backends::all());
        let adapter = adapters.iter().filter(|adapter| adapter.is_surface_supported(&surface)).next().expect("Failed to get first adapter");
        let (device, queue) = pollster::block_on(adapter.request_device(&DeviceDescriptor::default(), None)).unwrap();
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = *surface_capabilities.formats.iter().filter(|format| format.is_srgb()).next().expect("Failed to get SRGB texture format");

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let backend = Backend::new(surface, device, queue, config);

        self.state = Some(State {
            window,
            backend
        });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let Some(state) = &mut self.state else { return };
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::Resized(size) => {
                let Some(width) = NonZeroU32::new(size.width) else { return };
                let Some(height) = NonZeroU32::new(size.height) else { return };
                state.backend.resize(width, height);
            },
            WindowEvent::RedrawRequested => {
                state.backend.render();
            },
            _ => {}
        }
    }
}

impl<'a> Default for Application<'a> {
    fn default() -> Self {
        Self {
            state: None
        }
    }
}

fn main() {
    let mut event_loop = EventLoop::new().expect("Could not create event loop");
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = Application::default();
    event_loop.run_app(&mut app).unwrap();
}