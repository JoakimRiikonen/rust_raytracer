use std::ops::{Add, Div, Mul, Neg, Sub};

pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vec3 {
  pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
  }

  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  fn length_squared(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
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
}

impl Add for Vec3 {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Add<&Vec3> for &Vec3 {
  type Output = Vec3;

  fn add(self, other: &Vec3) -> Self::Output {
    Vec3 {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Sub for Vec3 {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

impl Sub<&Vec3> for &Vec3 {
  type Output = Vec3;

  fn sub(self, other: &Vec3) -> Self::Output {
    Vec3 {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
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

pub type Point = Vec3;