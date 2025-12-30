pub fn linspace(start: f32, end: f32, n: usize) -> Vec<f32> {
  if n == 0 {
    return Vec::new();
  }

  if n == 1 {
    return vec![start];
  }

  let step = (end - start) / (n - 1) as f32;
  (0..n).map(|i| start + step * i as f32).collect()
}

use std::f32::consts::PI;

fn _sin_taylor(v: f32) -> f32 {
  // 你原本的 5 阶泰勒展开公式
  v - (1.0 / 6.0) * v.powi(3) + (1.0 / 120.0) * v.powi(5)
}

fn _sin(mut x: f32) -> f32 {
  let two_pi = 2.0 * PI;

  // 1. 将 x 映射到 [0, 2pi]
  x = x % two_pi;
  if x < 0.0 {
    x += two_pi;
  }

  // 2. 利用对称性映射到 [-pi/2, pi/2]
  if x <= 0.5 * PI {
    _sin_taylor(x)
  } else if x <= 1.5 * PI {
    // 在 [pi/2, 3pi/2] 区间内，利用 sin(pi - x)
    _sin_taylor(PI - x)
  } else {
    // 在 [3pi/2, 2pi] 区间内，利用 sin(x - 2pi)
    _sin_taylor(x - two_pi)
  }
}

pub fn sin(x: &[f32]) -> Vec<f32> {
  x.iter().map(|&v| _sin(v)).collect()
}
