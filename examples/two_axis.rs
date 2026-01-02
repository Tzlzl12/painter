use std::{rc::Rc, thread::yield_now};

use painter::{
  Config, Figure,
  primitive::{self, Area, Curve, Stair},
  utils,
};

fn main() {
  let mut figure = Figure::new(Config::default());

  figure.add_subplot((1, 2));

  let c = Curve::new("y = sinx".to_string(), primitive::Config::default());
  let c1 = Curve::new("test".to_string(), primitive::Config::default());
  let t = utils::linspace(0.0, 6.28, 100);
  let y = utils::sin(&t);
  c.set_data(&t, &y);

  c1.set_fn(&t, |v| v - 3.14);
  let ax1 = figure.nth(0).unwrap();
  ax1.add(Rc::new(c));
  ax1.add(Rc::new(c1));
  ax1.set_strategy(painter::ScaleStrategy::Stretch);

  let stair = Stair::new("stair".to_string(), primitive::Config::default());
  stair.set_data(&[0., 1., 2., 3., 4., 5., 6.], &[4., 3., 7., 6., 1., 4., 1.]);

  let area_line = Area::new("line".to_string(), primitive::Config::default());
  area_line.set_data(&[0., 1., 2., 3., 4., 5., 6.], &[0., 8., 4., 1., 7., 7., 3.]);
  let ax2 = figure.nth(1).unwrap();
  ax2.add(Rc::new(stair));
  ax2.add(Rc::new(area_line));

  figure.show();
}
