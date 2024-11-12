use std::rc::Rc;
use std::thread;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopBuilder};
use winit::platform::windows::{EventLoopBuilderExtWindows, WindowBorrowExtWindows};
use winit::window::WindowId;

struct State {
    window: Rc<winit::window::Window>
}

impl State {
    fn new(event_loop: &ActiveEventLoop) -> Self {
        Self {
            window: Rc::new(event_loop.create_window(Default::default()).unwrap())
        }
    }
}

struct Backend {
    state: Option<State>,
    event_loop: Option<EventLoop<()>>
}

impl ApplicationHandler for Backend {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.state = Some(State::new(event_loop));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let Some(state) = &self.state else { return };
        match event {
            WindowEvent::CloseRequested => {
                panic!("Application closing");
            },
            _ => {}
        }
    }
}

impl Backend {
    pub fn new() -> Self {
        Self { 
            state: None,
            event_loop: None
        }
    }
    
    #[cfg(target_os = "windows")]
    pub fn run(&mut self) {
        unsafe {
            let event_loop = EventLoopBuilder::default().with_any_thread(true).build().unwrap();
            event_loop.set_control_flow(ControlFlow::Wait);
            event_loop.run_app(self).unwrap()
        }
    }
}

fn main() {
    let window_thread = thread::spawn(|| {
        let mut window = Backend::new();
        window.run();
    });
    
    println!("Window thread spawned");
    window_thread.join();
}