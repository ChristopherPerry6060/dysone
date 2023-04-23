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

use glium::glutin::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Fullscreen,
};
use glium::implement_vertex;
use glium::index;
use glium::uniform;
use glium::Frame;
use glium::Program;
use glium::Surface;
use glium::VertexBuffer;

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

/// Nanoseconds per frame for 60 FPS.
const SIXTY_FPS_FRAME_TIME: u64 = 16_666_667;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

// Convenience implementation.
impl From<[f32; 2]> for Vertex {
    fn from(value: [f32; 2]) -> Self {
        Vertex { position: value }
    }
}

// This implements glium's Vertex trait for the type.
// position matches the struct's field name.
// Not sure why this is needed yet.
implement_vertex!(Vertex, position);

fn main() {
    // Event Loop
    let el = event_loop::EventLoop::new();

    // Window Builder
    let wb = glium::glutin::window::WindowBuilder::new().with_title("Dysone");

    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &el).unwrap();

    el.run(move |event, _, control_flow| {
        let vertex1: Vertex = [-0.5, -0.5].into();
        let vertex2: Vertex = [0.0, 0.5].into();
        let vertex3: Vertex = [0.5, -0.5].into();

        // A triangle consists of 3 points, each with a set of coords.
        let shape = vec![vertex1, vertex2, vertex3];

        // Vertex buffer loads the shape into video card memory.
        // According to docs this is not required, just faster and trivial implement
        // due to the API.
        let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();

        // Copying this directly from docs, no idea why it is here.
        // It is supposed to be relevant once shapes grow in complexity.
        let indices = index::NoIndices(index::PrimitiveType::TrianglesList);

        let mut target = display.draw();
        draw_black(&mut target);
        target
            .draw(
                &vertex_buffer,
                indices,
                &get_program(&display),
                &EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

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

/// Builds a new program from GLSL source code.
///
/// This function is a wrapper around [`Program::from_source`] where the shaders
/// are populated with predefined constants. A Geometry shader is omitted.
fn get_program(display: &glium::Display) -> Program {
    Program::from_source(display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap()
}

/// Creates a black blackground.
fn draw_black(frame: &mut Frame) {
    frame.clear_color(0.0, 0.0, 0.0, 0.0);
}

/// Returns an instant describing when the next frame should be shown.
///
/// This is implemented using the `const` [`SIXTY_FPS_FRAME_TIME`].
fn next_frame_time() -> Instant {
    Instant::now() + Duration::from_nanos(SIXTY_FPS_FRAME_TIME)
}
