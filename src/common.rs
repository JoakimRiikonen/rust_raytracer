use std::f64::{consts::PI, INFINITY};

pub struct Common {
}

impl Common {
  pub const PI: f64 = PI;
  pub const INFINITY: f64 = INFINITY;

  pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * Common::PI / 180.0;
  }
}