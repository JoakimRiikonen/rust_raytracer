use crate::common::Common;

pub struct Interval {
  pub min: f64,
  pub max: f64,
}

impl Interval {
  pub const EMPTY: Interval = Interval {
    min: Common::INFINITY,
    max: -Common::INFINITY
  };

  pub const UNIVERSE: Interval = Interval {
    min: -Common::INFINITY,
    max: Common::INFINITY
  };

  pub fn new() -> Interval {
    Interval {
      min: -Common::INFINITY,
      max: Common::INFINITY,
    }
  }

  pub fn new_from_range(min: f64, max: f64) -> Interval {
    Interval {
      min,
      max
    }
  }

  pub fn contains(&self, x: f64) -> bool {
    self.min <= x && x <= self.max
  }

  pub fn surrounds(&self, x: f64) -> bool {
    self.min < x && x < self.max
  }
}