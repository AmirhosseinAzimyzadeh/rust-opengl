#[macro_use]
extern crate glium;
extern crate image;

mod vertex;
mod normal;
mod teapot;
mod math;

use glium::glutin::{event::Event, event_loop::{ControlFlow, EventLoopWindowTarget}};
use std::{ f32::consts::PI};

fn main() {
  use glium::glutin;

  let params = glium::DrawParameters {
    multisampling: true,
    depth: glium::Depth {
      test: glium::draw_parameters::DepthTest::IfLess,
      write: true,
      .. Default::default()
    },
    smooth: Some(glium::draw_parameters::Smooth::Nicest),
    .. Default::default()
  };

  let event_loop = glutin::event_loop::EventLoop::new();
  let window_builder = glutin::window::WindowBuilder::new();
  let context_builder = glutin::ContextBuilder::new().with_depth_buffer(24);
  let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();


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
        uniform mat4 perspective;
        void main() {
          v_normal = transpose(inverse(mat3(matrix))) * normal;
          gl_Position = perspective * matrix * vec4(position, 1.0);
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
    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

    let base: math::Mat4 = [
      [0.01, 0.0, 0.0, 0.0],
      [0.0, 0.01, 0.0, 0.0],
      [0.0, 0.0, 0.01, 0.0],
      [0.0, 0.0, 2.0, 1.0f32]
    ];

    let perspective = {
      let (width, height) = target.get_dimensions();
      let aspect_ratio = height as f32 / width as f32;
      let fov: f32 = 3.141592 / 3.0;
      let zfar = 1024.0;
      let znear = 0.1;
      let f = 1.0 / (fov / 2.0).tan();
      [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar+znear)/(zfar-znear), 1.0],
        [0.0, 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0],
      ]
    };

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
      perspective: perspective,
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
