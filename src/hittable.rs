use crate::{interval::Interval, material::Material, ray::Ray, vec3::{Point3, Vec3}};

pub struct HitRecord<'a> {
  pub p: Point3,
  pub normal: Vec3,
  pub material: &'a Box<dyn Material>,
  pub t: f64,
  pub front_face: bool,
}

impl<'a> HitRecord<'a> {
  pub fn set_face_normal(&self, ray: &Ray, outward_normal: Vec3) -> HitRecord<'a> {
    let front_face = ray.direction.dot(&outward_normal) < 0.0;
    let normal = if front_face { outward_normal } else { -outward_normal };
    HitRecord {
      p: self.p,
      t: self.t,
      material: self.material,
      normal,
      front_face,
    }
  }
}

pub trait Hittable {
  fn hit(&self, ray: &Ray, ray_t: &Interval) -> (bool, Option<HitRecord>);
}