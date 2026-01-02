use std::rc::Rc;

use painter::{
  Config, Figure,
  primitive::{self, Area, AreaType},
};

fn main() {
  let mut figure = Figure::new(Config::default());

  let x = [0., 1., 2., 3., 4., 5., 6.];
  let y = [0.0, 1., 2., 3., 4., 5.];

  let y1: Vec<f32> = y.iter().map(|&v| v + 1.).collect();
  let y2: Vec<f32> = y.iter().map(|&v| v * 2.).collect();

  let his = Rc::new(Area::new("test".to_string(), primitive::Config::default()));
  his.set_data(&x, &y);
  let his1 = Rc::new(Area::new("test1".to_string(), primitive::Config::default()));

  his1.set_data_norm(&y1);

  let mut area = Area::new("test2".to_string(), primitive::Config::default());
  area.area_type = AreaType::Line;
  let his2 = Rc::new(area);
  his2.set_data(&x, &[1., 4., 3., 7., 2., 7.]);

  let ax = figure.nth(0).unwrap();

  ax.add(his);
  ax.add(his1);
  ax.add(his2);
  ax.set_strategy(painter::ScaleStrategy::Stretch);

  figure.show();
}
