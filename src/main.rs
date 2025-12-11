mod app;
mod egui_tools;

use winit::event_loop::EventLoop;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(run());
    }
}

async fn run() {
    let event_loop = EventLoop::new().unwrap();

    let mut app = app::App::new();

    event_loop.run_app(&mut app).expect("Failed to run app");
}
