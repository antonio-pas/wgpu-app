mod app;

use app::run;
use winit::{event_loop::{EventLoop, ControlFlow}, window::{Window, WindowBuilder}};

async fn run_wrapper(window: Window, event_loop: EventLoop<()>) {
    let result = run(window, event_loop).await;
    if let Err(e) = result {
        panic!("error running app: {}", e);
    }
}
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start_wasm() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().expect("failed to initialize logger");
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut builder = WindowBuilder::new();
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowBuilderExtWebSys;
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        builder = builder.with_canvas(Some(canvas));
    let window = builder.build(&event_loop).unwrap();
        wasm_bindgen_futures::spawn_local(run_wrapper(window, event_loop));
}
