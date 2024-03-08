use crate::vec3::{Point3, Vec3};

pub struct Ray<'a> {
  pub origin: &'a Point3,
  pub direction: &'a Vec3,
}

impl Ray<'_> {
  pub fn new<'a>(origin: &'a Point3, direction: &'a Vec3) -> Ray<'a> {
    Ray { origin, direction }
  }

  pub fn at(&self, t: f64) -> Point3 {
    self.origin + &(self.direction * t)
  }
}