pub type Vec4 = [f32;4];
pub type Mat4 = [Vec4;4];

pub fn rotate_x(radian: f32) -> Mat4 {
  let mut result = [[0.0;4];4];
  result[0][0] = 1.0;
  result[1][1] = radian.cos();
  result[1][2] = radian.sin();
  result[2][1] = -radian.sin();
  result[2][2] = radian.cos();
  result[3][3] = 1.0;
  result
}

pub fn rotate_y(radian: f32) -> Mat4 {
  let mut result = [[0.0;4];4];
  result[0][0] = radian.cos();
  result[0][2] = -radian.sin();
  result[1][1] = 1.0;
  result[2][0] = radian.sin();
  result[2][2] = radian.cos();
  result[3][3] = 1.0;
  result
}

pub fn rotate_z(radian: f32) -> Mat4 {
  let mut result = [[0.0;4];4];
  result[0][0] = radian.cos();
  result[0][1] = radian.sin();
  result[1][0] = -radian.sin();
  result[1][1] = radian.cos();
  result[2][2] = 1.0;
  result[3][3] = 1.0;
  result
}

pub fn mat4_multiply(lfh: Mat4, rhs: Mat4) -> Mat4 {
  let mut result = [[0.0;4];4];
  for i in 0..4 {
    for j in 0..4 {
      result[i][j] = vec4_multiply(lfh[i], rhs[j]);
    }
  }
  result
}

pub fn vec4_multiply(lfh: Vec4, rhs: Vec4) -> f32 {
  let mut result = 0.0;
  for i in 0..4 {
    result += lfh[i] * rhs[i];
  }
  result
}