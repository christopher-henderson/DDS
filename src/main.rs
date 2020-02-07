use glium::backend::glutin::glutin::dpi::LogicalSize;
use glium::backend::glutin::glutin::ControlFlow;
use glium::glutin;

fn main() {
    // 1. The **winit::EventsLoop** for handling events.
    let mut events_loop = glium::glutin::EventsLoop::new();
    // 2. Parameters for building the Window.
    let wb = glium::glutin::WindowBuilder::new()
        .with_dimensions(LogicalSize{width: 1024 as f64, height: 768 as f64 })
        .with_title("Hello world");
    // 3. Parameters for building the OpenGL context.
    let cb = glium::glutin::ContextBuilder::new();
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();
    events_loop.run_forever(|event| {
        match event {
            glutin::Event::WindowEvent {window_id: id, event: glutin::WindowEvent::CloseRequested} => {
                ControlFlow::Break
            },
            _ => ControlFlow::Continue
        }
    });
}
