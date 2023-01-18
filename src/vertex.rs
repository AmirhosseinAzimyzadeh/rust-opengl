#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32),
}

impl Vertex {
    pub fn new(position: (f32, f32, f32)) -> Self {
    Vertex { position }
  }
}

implement_vertex!(Vertex, position);
