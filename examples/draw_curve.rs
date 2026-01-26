use painter::{
  Config, Figure,
  primitive::{self, Curve},
  utils,
};

fn main() {
  let mut figure = Figure::new(Config::default());
  figure.add_subplot((1, 1));
  let ax = figure.nth(0).unwrap();

  // 线 2: 参数方程
  let mut curve2 = Curve::new("Circle".to_string(), primitive::Config::default());
  let t = utils::linspace(0.0, 6.28, 100);
  curve2.set_parametric(&t, |v| v.cos(), |v| v.sin());
  ax.add(Box::new(curve2));
  // 线 1: 函数生成
  let mut curve1 = Curve::new("Sine".to_string(), primitive::Config::default());
  let x = utils::linspace(3.14, 6.28, 100);
  curve1.set_fn(&x, |v| 3. * v.sin());
  ax.add(Box::new(curve1));

  // 线 3: 直接喂数据
  let mut curve3 = Curve::new("Data".to_string(), primitive::Config::default());
  curve3.add_data(&[0.0, 2.0, 4.0], &[0.5, 0.8, 0.2]);
  ax.add(Box::new(curve3));

  figure.show();
}
