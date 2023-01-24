use crate::math;

pub fn read_shader(path: &str) -> String {
  use std::fs::File;
  use std::io::prelude::*;
  use std::io::BufReader;

  let file = File::open(path).unwrap();
  let mut reader = BufReader::new(file);
  let mut contents = String::new();
  reader.read_to_string(&mut contents).unwrap();
  contents
}


pub fn perspective_matrix((width, height): (u32, u32)) -> math::Mat4 {
  let aspect_ratio = height as f32 / width as f32;
  let fov: f32 = 3.141592 / 3.0;
  let z_far = 1024.0;
  let z_near = 0.1;
  let f = 1.0 / (fov / 2.0).tan();
  [
    [f * aspect_ratio, 0.0, 0.0, 0.0],
    [0.0, f, 0.0, 0.0],
    [0.0, 0.0, (z_far+z_near)/(z_far-z_near), 1.0],
    [0.0, 0.0, -(2.0*z_far*z_near)/(z_far-z_near), 0.0],
  ]
}