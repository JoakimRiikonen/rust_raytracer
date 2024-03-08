use crate::{interval::Interval, ray::Ray, vec3::{Point3, Vec3}};

pub struct HitRecord {
  pub p: Point3,
  pub normal: Vec3,
  pub t: f64,
  pub front_face: bool,
}

impl HitRecord {
  pub fn set_face_normal(&self, ray: &Ray, outward_normal: Vec3) -> HitRecord {
    let front_face = ray.direction.dot(&outward_normal) < 0.0;
    let normal = if front_face { outward_normal } else { -outward_normal };
    HitRecord {
      p: self.p,
      t: self.t,
      normal,
      front_face
    }
  }
}

pub trait Hittable {
  fn hit(&self, ray: &Ray, ray_t: &Interval) -> (bool, Option<HitRecord>);
}