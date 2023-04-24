use std::time::{Duration, Instant};

use glium::implement_vertex;
use glium::index;
use glium::Program;
use glium::Surface;
use glium::VertexBuffer;
use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::ControlFlow,
        window::Fullscreen,
    },
    uniforms::EmptyUniforms,
};

use dysone::util;
/// Main function is called *once per Vertex*.
#[allow(unused)]
const VERTEX_SHADER_SRC: &str = r#"
    #version 140
    in vec2 position;
    uniform float t;

    void main() { 
        vec2 pos = position;
        pos.x += t;
        gl_Position = vec4(pos, 0.0, 1.0);
    }
"#;

/// Main function is ran for each pixel that is drawn.
///
/// The color conventions is RGBA.
#[allow(unused)]
const FRAGMENT_SHADER_SRC: &str = r#"
    #version 140
    out vec4 color;

    void main() { 
        color = vec4(1.0, 0.0, 0.0, 1.0); 
    }
"#;

/// Nanoseconds per frame for 60 FPS.
const SIXTY_FPS_FRAME_TIME: u64 = 16_666_667;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

#[derive(Copy, Clone)]
struct Square {
    position: [[f32; 2]; 2],
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
    let el = glium::glutin::event_loop::EventLoop::new();
    let secondary = util::secondary_monitor_handle(&el);
    let fullscreen = Some(Fullscreen::Borderless(secondary));

    // Window Builder
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_fullscreen(fullscreen)
        .with_title("Dysone");

    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &el).unwrap();

    // Shape stuff
    let vertex1: Vertex = [-0.25, 0.0].into();
    let vertex2: Vertex = [0.0, 0.5].into();
    let vertex3: Vertex = [0.25, 0.0].into();
    let vertex2a: Vertex = [0.0, -0.5].into();
    let indices = index::NoIndices(index::PrimitiveType::TrianglesList);

    let triangle1 = vec![vertex1, vertex2, vertex3];

    let vertex_buffer1 = VertexBuffer::new(&display, &triangle1).unwrap();

    // A triangle consists of 3 points, each with a set of coords.
    let triangle2 = vec![vertex1, vertex2a, vertex3];
    let vertex_buffer2 = VertexBuffer::new(&display, &triangle2).unwrap();

    el.run(move |event, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target
            .draw(
                &vertex_buffer1,
                indices,
                &get_program(&display),
                &EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target
            .draw(
                &vertex_buffer2,
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

/// Returns an instant describing when the next frame should be shown.
///
/// This is implemented using the `const` [`SIXTY_FPS_FRAME_TIME`].
fn next_frame_time() -> Instant {
    Instant::now() + Duration::from_nanos(SIXTY_FPS_FRAME_TIME)
}
