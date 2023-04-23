use std::time::{Duration, Instant};

use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{self, ControlFlow},
    },
    Surface,
};

///
const SIXTY_FPS_FRAME_TIME: u64 = 16_666_667;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

// This implements glium's Vertex trait for the type.
// position matches the struct's field name.
// Not sure why this is needed yet.
implement_vertex!(Vertex, position);

#[allow(unused)]
fn main() {
    // Event Loop
    let mut el = event_loop::EventLoop::new();

    // Window Builder
    let mut wb = glium::glutin::window::WindowBuilder::new().with_title("Dysone");

    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &el).unwrap();

    el.run(move |event, _, control_flow| {
        draw_black(&display);

        // Control flow waits until next frame time.
        *control_flow = ControlFlow::WaitUntil(next_frame_time());

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        };
    });
}

/// Draw a black screen.
fn draw_black(display: &glium::Display) {
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 0.0);
    target.finish().unwrap();
}

fn next_frame_time() -> Instant {
    Instant::now() + Duration::from_nanos(SIXTY_FPS_FRAME_TIME)
}
