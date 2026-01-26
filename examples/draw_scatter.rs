use painter::{
  Config, Figure,
  primitive::{self, Scatter},
};

fn main() {
  let mut figure = Figure::new(Config::default());

  let mut scatter = Scatter::new("test".to_string(), primitive::Config::default());

  let n = [0., 1., 2., 3., 4., 5., 6., 7., 8., 9.];
  let y = [4., 3., 1., 6., 4., 3., 2., 1., 2., 3.];

  scatter.set_x(&n);
  scatter.set_y(&y);

  let mut s = Scatter::new("name".to_string(), primitive::Config::default());
  s.set_x(&n);
  s.set_y(&y.iter().map(|v| v + 1.).collect::<Vec<f32>>());

  let ax = figure.nth(0).unwrap();

  ax.add(Box::new(scatter));
  ax.add(Box::new(s));
  ax.set_strategy(painter::ScaleStrategy::Stretch);

  figure.show();
}
