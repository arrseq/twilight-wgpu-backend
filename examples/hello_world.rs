#![feature(mpmc_channel)]

use std::rc::Rc;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpmc::channel;
use std::thread;
use std::time::Duration;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};
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
    event_loop: Option<EventLoop<Command>>
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum Command {
    #[default]
    Stop
}

impl ApplicationHandler<Command> for Backend {
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

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: Command) {
        let Some(state) = &self.state else { return };
        match event {
            Command::Stop => {
                event_loop.exit()
            }
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
    
    fn load_event_loop(&mut self) {
        if let Some(_) = self.event_loop { return }
        let event_loop = EventLoopBuilder::default().with_any_thread(true).build().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);
        self.event_loop = Some(event_loop);
    }
    
    pub fn prepare_event_loop(&mut self) -> Result<EventLoopProxy<Command>, ()> {
        self.load_event_loop();
        Ok(self.event_loop.as_ref().ok_or(())?.create_proxy())
    }
    
    #[cfg(target_os = "windows")]
    pub fn run(&mut self) {
        let Some(event_loop) = self.event_loop.take() else { return };
        event_loop.run_app(self).unwrap()
    }
}

fn main() {
    let (sender, receiver) = channel();
    let window_thread = thread::spawn(move || {
        let mut window = Backend::new();
        sender.send(window.prepare_event_loop().unwrap()).unwrap();
        window.run();
    });
    
    println!("Window thread spawned");
    
    thread::sleep(Duration::from_millis(2000));
    let proxy = receiver.recv().unwrap();
    
    proxy.send_event(Command::Stop);
    
    window_thread.join();
}