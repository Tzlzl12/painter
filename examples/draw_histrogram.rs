use std::rc::Rc;

use painter::{Config, Figure, primitive::Histrogram};

fn main() {
  let mut figure = Figure::new(Config::default());

  let his = Histrogram::new("test".to_string());
  let t: [f32; _] = [0., 1., 2., 3., 4., 5., 6., 7., 8.];
  // rand value five
  let value: [f32; _] = [3., 5., 6., 2., 1., 2., 7., 1.];
  his.set_x(&t);
  his.set_data(&value);
  let value1 = value.iter().map(|&v| v * 2.).collect::<Vec<f32>>();
  his.set_data_prototype(&value1, 0., 1.);

  let ax = figure.nth(0).unwrap();
  ax.add(Rc::new(his));
  ax.set_strategy(painter::ScaleStrategy::Stretch);
  figure.show();
}
