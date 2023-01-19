#[macro_use]
extern crate glium;
extern crate image;

mod vertex;
mod normal;
mod teapot;
mod math;

use glium::glutin::{event::Event, event_loop::{ControlFlow, EventLoopWindowTarget}};
use vertex::Vertex;
use std::{io::Cursor, f32::consts::PI};

fn main() {
  use glium::glutin;

  let params = glium::DrawParameters {
    multisampling: true,
    smooth: Some(glium::draw_parameters::Smooth::Nicest),
    .. Default::default()
  };

  let event_loop = glutin::event_loop::EventLoop::new();
  let window_builder = glutin::window::WindowBuilder::new();
  let context_builder = glutin::ContextBuilder::new();
  let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

  // load image
  let image = image::load(
      Cursor::new(&include_bytes!("..\\assets\\texture.jpg")),
      image::ImageFormat::Jpeg,
  ).unwrap().to_rgb8();

  let dimention = image.dimensions();
  // let image = glium::texture::RawImage2d::from_raw_rgb_reversed(&image.into_raw(), dimention);

  let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
  let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
  let indices = glium::IndexBuffer::new(
          &display,
          glium::index::PrimitiveType::TrianglesList,
          &teapot::INDICES).unwrap();

  let vertex_shader_src = r#"
        #version 140
        in vec3 position;
        in vec3 normal;
        out vec3 v_normal;
        uniform mat4 matrix;
        void main() {
          v_normal = transpose((mat3(matrix))) * normal;
          gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

  let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        in vec3 v_normal;
        uniform vec3 u_light;
        void main() {
          float brightness = dot(normalize(v_normal), normalize(u_light));
          vec3 dark_color = vec3(0.4, 0.0, 0.0);
          vec3 regular_color = vec3(1.0, 0.0, 0.0);
          color = vec4(mix(dark_color, regular_color, brightness), 1.0);
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
    target.clear_color(0.0, 0.0, 0.0, 1.0);

    let base: math::Mat4 = [
      [0.01, 0.0, 0.0, 0.0],
      [0.0, 0.01, 0.0, 0.0],
      [0.0, 0.0, 0.01, 0.0],
      [0.0, 0.0, 0.0, 1.0f32]
    ];

    let matrix = math::mat4_multiply(
      math::rotate_y(time_step),
      base
    );

    let matrix = math::mat4_multiply(
      math::rotate_x(time_step),
      matrix
    );

    let matrix = math::mat4_multiply(
      math::rotate_z(time_step),
      matrix
    );

    let light = [-1.0, 0.4, 0.9f32];

    let uniforms = uniform! {
      matrix: matrix,
      u_light: light,
    };

    target.draw(
      (&positions, &normals),
      &indices,
      &program,
      &uniforms,
      &params,
    ).unwrap();

    target.finish().unwrap();

    time_step += 0.0002;
    if time_step > (2.0 * PI) { time_step = -0.5; }
  });
}

fn loop_handler(
  event: Event<()>,
  _: &EventLoopWindowTarget<()>,
  control_flow: &mut ControlFlow,
) {
  use glium::glutin;
  // let next_frame_time = std::time::Instant::now()
  //   + std::time::Duration::from_nanos(16_666_667);

  // *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
  
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
