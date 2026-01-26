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

  let mut his = Area::new("test".to_string(), primitive::Config::default());
  his.set_data(&x, &y);

  let mut his1 = Area::new("test1".to_string(), primitive::Config::default());
  his1.set_data_norm(&y1);

  let mut his2 = Area::new("test2".to_string(), primitive::Config::default());
  his2.change_area_type(AreaType::Line);
  his2.set_data(&x, &[1., 4., 3., 7., 2., 7.]);

  let ax = figure.nth(0).unwrap();

  ax.add(Box::new(his));
  ax.add(Box::new(his1));
  ax.add(Box::new(his2));
  ax.set_strategy(painter::ScaleStrategy::Stretch);

  figure.show();
}
