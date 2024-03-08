use std::f64::{consts::PI, INFINITY};

pub struct Common {
}

impl Common {
  pub const PI: f64 = PI;
  pub const INFINITY: f64 = INFINITY;

  pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * Common::PI / 180.0;
  }

  // Return a random float in range [0,1)
  pub fn random_float() -> f64 {
    rand::random::<f64>()
  }

  pub fn random_float_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * Common::random_float()
  }
}