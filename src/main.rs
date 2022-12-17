mod vertex;

use glium::{glutin::{event::Event, event_loop::{ControlFlow, EventLoopWindowTarget}}, Display};
use vertex::Vertex;

fn main() {
  use glium::glutin;

  let event_loop = glutin::event_loop::EventLoop::new();
  let window_builder = glutin::window::WindowBuilder::new();
  let context_builder = glutin::ContextBuilder::new();
  let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

  event_loop.run(move |e, t, cf| {
    loop_handler(e, t, cf, &display)
  });
}

fn loop_handler(
  event: Event<()>,
  _: &EventLoopWindowTarget<()>,
  control_flow: &mut ControlFlow,
  display: &Display,
) {
  use glium::{glutin, Surface};
  let next_frame_time = std::time::Instant::now()
    + std::time::Duration::from_nanos(16_666_667);

  *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

  let mut target = display.draw();
  target.clear_color(1.0, 0.0, 0.0, 1.0);
  target.finish().unwrap();
  
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
