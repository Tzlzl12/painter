use painter::{
  Config, Figure,
  primitive::{ErrorBar, Histrogram},
};

fn main() {
  let mut figure = Figure::new(Config::default());

  let mut his = Histrogram::new("test".to_string());
  let t: [f32; _] = [0., 1., 2., 3., 4., 5., 6., 7., 8.];
  // rand value five
  let value: [f32; _] = [3., 5., 6., 2., 1., 2., 7., 1.];
  his.set_x(&t);
  his.set_data(&value);
  let value1 = value.iter().map(|&v| v * 2.).collect::<Vec<f32>>();
  his.set_data(&value1);

  let mut eb = ErrorBar::new("test".to_string());
  eb.from_histogram(&his);

  let ax = figure.nth(0).unwrap();
  ax.add(Box::new(his));
  ax.add(Box::new(eb));
  ax.set_strategy(painter::ScaleStrategy::Stretch);
  figure.show();
}
