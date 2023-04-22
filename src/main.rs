use std::time::{Duration, Instant};

use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{self, ControlFlow},
    },
    Surface,
};

const NANOS: u64 = 16_666_667;

#[allow(unused)]
fn main() {
    // Event Loop
    let mut el = event_loop::EventLoop::new();

    // Window Builder
    let mut wb = glium::glutin::window::WindowBuilder::new().with_title("Dyson");

    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &el).unwrap();

    el.run(move |ev, _, control_flow| {
        draw_black(&display);

        // Control flow waits until next frame time.
        *control_flow = ControlFlow::WaitUntil(next_frame_time());

        // Check for window event?
        if let Event::WindowEvent { event, .. } = ev {
            // This is needed for ALT F4
            if let WindowEvent::CloseRequested = event {
                *control_flow = ControlFlow::Exit;
                return;
            };
        } else {
            // This is where the good stuff goes?
            return;
        }
    });
}

/// Draw a black screen.
fn draw_black(display: &glium::Display) {
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 0.0);
    target.finish().unwrap();
}

fn next_frame_time() -> Instant {
    Instant::now() + Duration::from_nanos(NANOS)
}
