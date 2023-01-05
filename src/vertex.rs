#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    texture_coordinate: [f32; 2],
}

impl Vertex {
  pub fn new(position: [f32; 2], texture_coordinate: [f32; 2]) -> Self {
    Vertex { position, texture_coordinate }
  }
}

implement_vertex!(Vertex, position, texture_coordinate);
