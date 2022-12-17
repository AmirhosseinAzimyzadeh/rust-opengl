#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    pub fn new(position: [f32; 2]) -> Self {
      Vertex { position }
    }
}
