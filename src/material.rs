use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray);
}

pub struct Lambertian {
  albedo: Color,
}

impl Lambertian {
  pub fn new(albedo: Color) -> Lambertian {
    Lambertian {
      albedo
    }
  }
}

impl Material for Lambertian {
  fn scatter(&self, _: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
    let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

    if scatter_direction.near_zero() {
      scatter_direction = rec.normal;
    }

    let scattered = Ray::new(rec.p, scatter_direction);
    return (true, self.albedo, scattered);
  }
}

pub struct Metal {
  albedo: Color,
  fuzz: f64,
}

impl Metal {
  pub fn new(albedo: Color, fuzz: f64) -> Metal {
    Metal {
      albedo,
      fuzz,
    }
  }
}

impl Material for Metal {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
    let reflected = r_in.direction.unit_vector().reflect(&rec.normal);
    let scattered = Ray::new(rec.p, reflected + Vec3::random_unit_vector() * self.fuzz);
    return (scattered.direction.dot(&rec.normal) > 0.0, self.albedo, scattered);
  }
}