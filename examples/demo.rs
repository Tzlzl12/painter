use anyhow::Result;

use painter::{Config, Figure};

fn main() -> Result<()> {
  let mut figure = Figure::new(Config::default());

  figure.add_subplot((1, 2));
  figure.show();

  Ok(())
}
