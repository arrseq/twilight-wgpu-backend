#![feature(mpmc_channel)]
#![feature(type_alias_impl_trait)]
#![feature(fn_traits)]

use std::default::Default;
use std::slice::Iter;
use std::sync::Arc;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;
use wgpu::{Backends, DeviceDescriptor, Instance, InstanceDescriptor, SurfaceConfiguration, TextureFormat, TextureUsages};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};
#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::platform::windows::WindowExtWindows;
use winit::window::WindowId;

struct State<'a> {
    window: Arc<winit::window::Window>,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}

pub struct Backend<'a, AdapterSelector, FormatSelector> {
    state: Option<State<'a>>,
    event_loop: Option<EventLoop<Command>>,
    backends: Backends,
    adapter_selector: AdapterSelector,
    format_selector: FormatSelector,
    // #[cfg(target_arch = "wasm32")]
    // canvas_element: String
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum Command {
    #[default]
    Stop
}

#[derive(Debug, Clone)]
pub struct ProxyRemote(EventLoopProxy<Command>);

impl ProxyRemote {
    pub fn stop(self) {
        self.0.send_event(Command::Stop).unwrap()
    }
}

pub mod adapter_selector {
    use std::slice::Iter;
    use wgpu::{Adapter, TextureFormat};

    pub type Adapters<'a> = impl Iterator<Item = &'a Adapter> + 'a;
    pub fn define_adapters<'a>(iter: Iter<'a, Adapter>, surface: &'a wgpu::Surface<'a>) -> Adapters<'a> {
        iter.filter(|adapter| adapter.is_surface_supported(surface))
    }

    pub type Formats<'a> = impl Iterator<Item = &'a TextureFormat> + 'a;
    pub fn define_formats<'a>(iter: Iter<'a, TextureFormat>) -> Formats<'a> {
        iter.filter(|format| format.is_srgb())
    }
}

impl<'a, AdapterSelector, FormatSelector> ApplicationHandler<Command> for Backend<'a, AdapterSelector, FormatSelector> where
    AdapterSelector: FnMut(adapter_selector::Adapters) -> usize,
    FormatSelector: FnMut(adapter_selector::Formats) -> usize {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(event_loop.create_window(Default::default()).unwrap());
        let size = window.inner_size();
        
        let instance = Instance::new(InstanceDescriptor {
            backends: self.backends,
            ..Default::default()
        });
        
        let surface = instance.create_surface(window.clone()).unwrap();
        
        let adapter = {
            let mut enumerated = instance.enumerate_adapters(self.backends);
            let index = self.adapter_selector.call_mut((adapter_selector::define_adapters(enumerated.iter(), &surface),));
            enumerated.swap_remove(index) // TODO: Handle invalid indexes
        };
        let (device, queue) = pollster::block_on(adapter.request_device(&DeviceDescriptor::default(), None)).unwrap();
        
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = {
            let index = self.format_selector.call_mut((adapter_selector::define_formats(surface_capabilities.formats.iter()),));
            surface_capabilities.formats.get(index).copied().unwrap()
        };
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
        
        self.state = Some(State {
            size: window.inner_size(),
            surface,
            device,
            queue,
            config,
            window
        });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let Some(state) = &self.state else { return };
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            _ => {}
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: Command) {
        let Some(state) = &self.state else { return };
        match event {
            Command::Stop => {
                event_loop.exit()
            }
        }
    }
}

impl<AdapterSelector, FormatSelector> Backend<'_, AdapterSelector, FormatSelector> where
    AdapterSelector: FnMut(adapter_selector::Adapters) -> usize,
    FormatSelector: FnMut(adapter_selector::Formats) -> usize {
    #[cfg(target_arch = "wasm32")]
    pub fn new(rendering_surface: HtmlCanvasElement) -> Self {
        Self {
            state: None,
            event_loop: None,
            // rendering_surface
        }
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(backends: Backends, adapter_selector: AdapterSelector, format_selector: FormatSelector) -> Self {
        Self {
            state: None,
            event_loop: None,
            adapter_selector,
            format_selector,
            backends
        }
    }

    fn load_event_loop(&mut self) {
        if let Some(_) = self.event_loop { return }
        let event_loop = EventLoopBuilder::default().with_any_thread(true).build().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);
        self.event_loop = Some(event_loop);
    }

    pub fn prepare_event_loop(&mut self) -> Result<ProxyRemote, ()> {
        self.load_event_loop();
        Ok(ProxyRemote(self.event_loop.as_ref().ok_or(())?.create_proxy()))
    }

    #[cfg(target_os = "windows")]
    pub fn run(&mut self) {
        let Some(event_loop) = self.event_loop.take() else { return };
        event_loop.run_app(self).unwrap()
    }
    
    /// # Result
    /// Returns [None] if the state has not been initialized yet. The event loop must be running for
    /// this to work correctly.
    pub fn window(&mut self) -> Option<&winit::window::Window> {
        let Some(state) = &self.state else { return None };
        Some(&state.window)
    }
}