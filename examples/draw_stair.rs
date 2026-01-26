use painter::Config;
use painter::Figure;
use painter::primitive::{self, Stair, StairStyle};

fn main() {
  let mut figure = Figure::new(Config::default());
  let ax = figure.nth(0).unwrap();

  // 1. 阶梯图 (模拟离散采样)
  let mut stair = Stair::new("Steps".to_string(), primitive::Config::default());

  stair.set_style(StairStyle::Histogram);
  stair.set_data(&[0.0, 1.0, 2.0, 3.0, 4.0], &[0.2, 2.3, 0.4, 0.9, 0.3]);
  ax.add(Box::new(stair));

  figure.show();
}
