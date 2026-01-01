use painter::Config;
use painter::Figure;
use painter::primitive::{self, Stair, StairStyle};
use std::rc::Rc;

fn main() {
  let mut figure = Figure::new(Config::default());
  let ax = figure.nth(0).unwrap();

  // 1. 阶梯图 (模拟离散采样)
  let stair = Rc::new(Stair::new(
    "Steps".to_string(),
    primitive::Config::default(),
  ));

  stair.set_style(StairStyle::histogram);
  stair.set_data(&[0.0, 1.0, 2.0, 3.0, 4.0], &[0.2, 0.8, 0.4, 0.9, 0.3]);
  ax.add(stair);

  figure.show();
}
