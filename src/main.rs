#[macro_use]
extern crate glium;

mod vertex;

use glium::glutin::{event::Event, event_loop::{ControlFlow, EventLoopWindowTarget}};
use vertex::Vertex;

fn main() {
  use glium::glutin;

  let event_loop = glutin::event_loop::EventLoop::new();
  let window_builder = glutin::window::WindowBuilder::new();
  let context_builder = glutin::ContextBuilder::new();
  let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

  let shape = vec![
    Vertex::new([0.5, 0.5]),
    Vertex::new([-0.5, -0.5]),
    Vertex::new([-0.5, 0.5]),
  ];

  let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
  let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

  let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        out vec2 out_attr;
        uniform mat4 matrix;
        void main() {
            out_attr = position;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

  let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        in vec2 out_attr;
        void main() {
            color = vec4(out_attr, 0.0, 1.0);
        }
    "#;

  let program = glium::Program::from_source(
      &display,
      vertex_shader_src,
      fragment_shader_src,
      None
  ).unwrap();


  let mut time_step:f32 = -0.5;
  event_loop.run(move |e, t, cf| {
    loop_handler(e, t, cf);

    use glium::Surface;

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);

    let uniforms = uniform! {
        matrix: [
          [time_step.cos(), time_step.sin(), 0.0, 0.0],
          [-time_step.sin(), time_step.cos(), 0.0, 0.0],
          [0.0, 0.0, 1.0, 0.0],
          [ 0.0 , 0.0, 0.0, 1.0f32],
        ]
    };

    target.draw(
      &vertex_buffer,
      &indices,
      &program,
      &uniforms,
      &Default::default()
    ).unwrap();

    target.finish().unwrap();

    time_step += 0.0002;
    if time_step > 0.5 { time_step = -0.5; }
  });
}

fn loop_handler(
  event: Event<()>,
  _: &EventLoopWindowTarget<()>,
  control_flow: &mut ControlFlow,
) {
  use glium::glutin;
  let next_frame_time = std::time::Instant::now()
    + std::time::Duration::from_nanos(16_666_667);

  *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
  
  match event {
    glutin::event::Event::WindowEvent { event, .. } => match event {
      glutin::event::WindowEvent::CloseRequested => {
        *control_flow = glutin::event_loop::ControlFlow::Exit;
          return;
        },
        _ => return,
    },
    _ => (),
  }
}
