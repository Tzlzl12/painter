use painter::Config;
use painter::Figure;
use painter::primitive::{self, Curve, Stair};
use painter::utils;
use std::rc::Rc;

fn main() {
  let mut figure = Figure::new(Config::default());
  let ax = figure.nth(0).unwrap();

  // 1. 阶梯图 (模拟离散采样)
  let stair = Rc::new(Stair::new(
    "Steps".to_string(),
    primitive::Config::default(),
  ));
  stair.set_data(&[0.0, 1.0, 2.0, 3.0, 4.0], &[0.2, 0.8, 0.4, 0.9, 0.3]);
  ax.add(stair);

  // // 2. 参数方程圆
  // let circle = Rc::new(Curve::new(
  //   "Circle".to_string(),
  //   primitive::Config::default(),
  // ));
  // let t = utils::linspace(0.0, 6.28, 100);
  // circle.set_parametric(&t, |v| v.cos(), |v| v.sin());
  // ax.add(circle);

  figure.show();
}
