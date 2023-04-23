// OpenGL does not use pixel coordinates but rather splits the screen
// to have a height and width of two units.
//
// * 0,0 is center
// * 0,1 is top-center
// * -1,-1 is bottom-left
//
// Floating points are used to express the range of possible points between
// the screens bounds.
use std::time::{Duration, Instant};

use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{self, ControlFlow},
    },
    implement_vertex, Surface,
};

/// This is GLSL which handles the vertex shading (and all shading in general).
///
/// This ties into the `implement_vertex` macro invocation. vec2 is a
/// GLSL?(maybe just C) equivalent to the Rust type `[T; 2]`^1.
/// `in vec2 position` matches the position field within the [`Vertex`] struct
///
/// The main function is called *once per Vertex*, or 3 times for a triangle.
///
/// [^1]: The docs states `vec2` is equivalent to `[f32; 2]`, I expect
/// they mean equivalent in length rather than composition.
#[allow(unused)]
const VERTEX_SHADER_SRC: &str = r#"
    #version 140
    in vec2 position;
    void main() { gl_Position = vec4(position, 0.0, 1.0); }
"#;

/// See related documentation for [`VERTEX_SHADER_SRC`].
///
/// contrary to the vertex shader, this main function is ran for each pixel
/// that is drawn.
///
/// The color conventions is RGBA.
#[allow(unused)]
const FRAGMENT_SHADER_SRC: &str = r#"
    #version 140
    out vec4 color;
    void main() { color = vec4(1.0, 0.0, 0.0, 1.0); }
"#;

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

/// Creates a black backbuffer and swaps it with the shown buffer.
fn draw_black(display: &glium::Display) {
    // calling draw on a display is not a mutable borrow
    // it is returning a new backbuffer instance.
    // Display is context from which you can request a frame buffer.
    let mut target = display.draw();

    // R G B Alpha chx
    target.clear_color(0.0, 0.0, 0.0, 0.0);
}

fn next_frame_time() -> Instant {
    Instant::now() + Duration::from_nanos(SIXTY_FPS_FRAME_TIME)
}
