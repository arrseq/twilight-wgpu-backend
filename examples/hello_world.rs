#![feature(portable_simd)]

use std::default::Default;
use std::iter;
use std::num::NonZeroU32;
use std::simd::Simd;
use std::sync::Arc;
use wgpu::{Backends, BufferDescriptor, BufferUsages, Color, CommandEncoder, CommandEncoderDescriptor, Device, DeviceDescriptor, Instance, InstanceDescriptor, LoadOp, Operations, PresentMode, Queue, RenderPassColorAttachment, RenderPassDescriptor, StoreOp, Surface, SurfaceConfiguration, SurfaceTexture, TextureUsages, TextureView, TextureViewDescriptor};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;
use twilight_wgpu_backend::output::object::{ObjectClass, ObjectShader};
use twilight_wgpu_backend::output::Output;

struct State<'a> {
    window: Arc<winit::window::Window>,
    color_it: bool,
    surface: Surface<'a>,
    config: SurfaceConfiguration,
    device: Device,
    out: Output,
    queue: Queue
}

struct Application<'a> {
    state: Option<State<'a>>
}

impl ApplicationHandler<()> for Application<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let None = self.state else { return };
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
            present_mode: PresentMode::AutoVsync,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let mut out = Output::new(&device, surface_format);

        {
            let layout = device.create_buffer(&BufferDescriptor {
                label: None,
                usage: BufferUsages::VERTEX,
                
            })
            
            out.object_classes.push(ObjectClass {
                shape_id: 0,
                shader: ObjectShader::Uniform(Simd::splat(1.0)),
                vertexes:
            });
        }

        self.state = Some(State {
            window,
            out,
            surface,
            config,
            device,
            queue,
            color_it: false
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
                state.config.width = width.into();
                state.config.height = height.into();
                state.surface.configure(&state.device, &state.config)
            },
            WindowEvent::MouseInput { state: mouse_state, ..  } => {
                state.color_it = mouse_state.is_pressed();
                state.window.request_redraw();
            },
            WindowEvent::RedrawRequested => {
                let output = state.surface.get_current_texture().expect("SF");
                let view = output.texture.create_view(&TextureViewDescriptor::default());

                let mut encoder = state.device.create_command_encoder(&CommandEncoderDescriptor {
                    label: None
                });
                
                let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: None,
                    color_attachments: &[
                        Some(RenderPassColorAttachment {
                            view: &view,
                            ops: Operations {
                                load: LoadOp::Clear(Color::TRANSPARENT),
                                store: StoreOp::Store
                            },
                            resolve_target: None
                        })
                    ],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });
                
                state.out.render(&mut pass);
                
                drop(pass);
                
                state.queue.submit(iter::once(encoder.finish()));
                output.present();
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

#[tokio::main]
async fn main() {
    let mut event_loop = EventLoop::new().expect("Could not create event loop");
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = Application::default();
    event_loop.run_app(&mut app).unwrap();
}