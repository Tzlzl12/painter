use painter::{
  Config, Figure,
  primitive::{self, Curve},
  utils,
};

fn main() {
  let mut figure = Figure::new(Config::default());

  let mut curve = Curve::new("test".to_string(), primitive::Config::default());

  let t = utils::linspace(0., 6.28, 100);
  let y: Vec<f32> = utils::sin(&t);
  curve.add_data(&t, &y);
  // println!("x: {:?} y: {:?}", t, y);
  let ax = figure.nth(0).unwrap();
  ax.add(Box::new(curve));
  // ax.set_x_limit(Some((0., 7.)));
  // ax.set_y_limit(Some((-1., 1.)));

  figure.show();
}
