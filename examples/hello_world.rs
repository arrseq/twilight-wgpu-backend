use twilight_wgpu_backend::Backend;
use wgpu::Backends;

fn main() {
    let mut backend = Backend::new(Backends::VULKAN, |_| { 0 }, |_| { 0 });
    backend.prepare_event_loop();
    backend.run();
}