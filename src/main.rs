mod app;
use app::run;
use winit::{window::WindowBuilder, event_loop::{EventLoop, ControlFlow}};

pub fn start_normal() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    pollster::block_on(run(window, event_loop)).expect("error");
}
fn main() {
    start_normal();
}
