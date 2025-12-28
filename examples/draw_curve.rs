use painter::{
  Config, Figure,
  primitive::{self, Curve},
};

fn main() {
  let mut figure = Figure::new(Config::default());

  let mut curve = Curve::new("test".to_string(), primitive::Config::default());

  let mut data = Vec::<(f32, f32)>::new();
  for i in 0..100 {
    let y = i as f32 * 2.5;
    data.push((i as f32, y));
  }
  curve.add_data(data);

  let ax = figure.nth(0).unwrap();
  ax.add(Box::new(curve));
  ax.set_x_limit((0., 110.));
  ax.set_y_limit((0., 270.));

  figure.show();
}
