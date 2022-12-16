use glium::glutin::{event::Event, event_loop::{ControlFlow, EventLoopWindowTarget}};

#[macro_use]
extern crate glutin;

fn main() {
  use glium::glutin;

  let mut event_loop = glutin::event_loop::EventLoop::new();
  let window_builder = glutin::window::WindowBuilder::new();
  let context_builder = glutin::ContextBuilder::new();
  let display = glium::Display::new(window_builder, context_builder, &event_loop);

  event_loop.run(loop_handler);
}

fn loop_handler(
  event: Event<()>,
  _: &EventLoopWindowTarget<()>,
  control_flow: &mut ControlFlow
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