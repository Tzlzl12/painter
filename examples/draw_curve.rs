use std::rc::Rc;

use painter::{
  Config, Figure,
  primitive::{self, Curve},
  utils,
};

fn main() {
  let mut figure = Figure::new(Config::default());
  figure.add_subplot((1, 1));
  let ax = figure.nth(0).unwrap();

  // 线 1: 函数生成
  let curve1 = Rc::new(Curve::new("Sine".to_string(), primitive::Config::default()));
  let x = utils::linspace(0.0, 6.28, 100);
  curve1.set_fn(&x, |v| v.sin());
  ax.add(curve1);

  // 线 2: 参数方程
  let curve2 = Rc::new(Curve::new(
    "Circle".to_string(),
    primitive::Config::default(),
  ));
  let t = utils::linspace(0.0, 6.28, 100);
  curve2.set_parametric(&t, |v| v.cos(), |v| v.sin());
  ax.add(curve2);

  // 线 3: 直接喂数据
  let curve3 = Rc::new(Curve::new("Data".to_string(), primitive::Config::default()));
  curve3.add_data(&[0.0, 2.0, 4.0], &[0.5, 0.8, 0.2]);
  ax.add(curve3);

  figure.show();
}
