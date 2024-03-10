use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::common::Common;

#[derive(Copy, Clone)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vec3 {
  pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
  }

  pub fn random() -> Vec3 {
    Vec3 {
      x: Common::random_float(),
      y: Common::random_float(),
      z: Common::random_float(),
    }
  }

  pub fn random_in_range(min: f64, max: f64) -> Vec3 {
    Vec3 {
      x: Common::random_float_in_range(min, max),
      y: Common::random_float_in_range(min, max),
      z: Common::random_float_in_range(min, max),
    }
  }

  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn length_squared(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn near_zero(&self) -> bool {
    let s = 1e-8;
    return self.x.abs() < s && self.y.abs() < s && self.z.abs() < s;
  }

  pub fn dot(&self, vec: &Vec3) -> f64 {
    self.x * vec.x + self.y * vec.y + self.z * vec.z
  }

  pub fn cross(&self, vec: &Vec3) -> Vec3 {
    Vec3 {
      x: self.y * vec.z - self.z * vec.y,
      y: self.z * vec.x - self.x * vec.z,
      z: self.x * vec.y - self.y * vec.x
    }
  }

  pub fn unit_vector(&self) -> Vec3 {
    self / self.length()
  }
  
  pub fn random_in_unit_sphere() -> Vec3 {
    loop {
      let p = Vec3::random_in_range(-1.0, 1.0);
      if p.length_squared() < 1.0 {
        return p;
      }
    }
  }

  pub fn random_unit_vector() -> Vec3 {
    return Vec3::random_in_unit_sphere().unit_vector();
  }

  pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = Vec3::random_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
      return on_unit_sphere;
    }
    return -on_unit_sphere;
  }

  pub fn reflect(&self, n: &Vec3) -> Vec3 {
    return self - &(n * self.dot(n) * 2.0);
  }

  pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = (-self).dot(n).min(1.0);
    let r_out_perp = (self + &(n * cos_theta)) * etai_over_etat;
    let inner = (1.0 - r_out_perp.length_squared()).abs();
    let r_out_parallel = n * (-(inner.sqrt()));
    return r_out_perp + r_out_parallel;
  }
}


impl Add for &Vec3 {
  type Output = Vec3;

  fn add(self, other: Self) -> Self::Output {
    Vec3 {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Add for Vec3 {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    &self + &other
  }
}

impl Add<&Vec3> for Vec3 {
  type Output = Vec3;

  fn add(self, other: &Vec3) -> Self::Output {
    &self + other
  }
}

impl Sub for &Vec3 {
  type Output = Vec3;

  fn sub(self, other: Self) -> Self::Output {
    Vec3 {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl Sub for Vec3 {
  type Output = Vec3;

  fn sub(self, other: Self) -> Self::Output {
    &self - &other
  }
}

impl Sub<&Vec3> for Vec3 {
  type Output = Vec3;

  fn sub(self, other: &Vec3) -> Self::Output {
    &self - other
  }
}

impl Neg for Vec3 {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}

impl Neg for &Vec3 {
  type Output = Vec3;

  fn neg(self) -> Self::Output {
    Vec3 {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}

impl Mul<f64> for Vec3 {
  type Output = Self;

  fn mul(self, rhs: f64) -> Self::Output {
    Self {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    }
  }
}

impl Mul<f64> for &Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: f64) -> Self::Output {
    Vec3 {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    }
  }
}

impl Mul<Vec3> for Vec3 {
  type Output = Vec3;

  fn mul(self, other: Vec3) -> Self::Output {
    Vec3 {
      x: self.x * other.x,
      y: self.y * other.y,
      z: self.z * other.z,
    }
  }
}

impl Div<f64> for Vec3 {
  type Output = Self;

  fn div(self, rhs: f64) -> Self::Output {
    self * (1.0/rhs)
  }
}

impl Div<f64> for &Vec3 {
  type Output = Vec3;

  fn div(self, rhs: f64) -> Self::Output {
    self * (1.0/rhs)
  }
}

pub type Point3 = Vec3;