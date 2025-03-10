use crate::{color::Color, common::Common, hittable::HitRecord, ray::Ray, vec3::Vec3};

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

pub struct Dielectric {
  ir: f64,
}

impl Dielectric {
  pub fn new(ir: f64) -> Dielectric {
    Dielectric {
      ir
    }
  }

  fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0*r0;
    return r0 + (1.0 - r0)*(1.0 - cosine).powi(5);
  }
}

impl Material for Dielectric {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
    let attenuation = Color::new(1.0, 1.0, 1.0);
    let refraction_ratio = if rec.front_face { 1.0/self.ir } else { self.ir };

    let unit_direction = r_in.direction.unit_vector();
    let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
    let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
    
    let cannot_refract = refraction_ratio * sin_theta > 1.0;
    let direction = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > Common::random_float() {
      unit_direction.reflect(&rec.normal)
    } else {
      unit_direction.refract(&rec.normal, refraction_ratio)
    };
    let scattered = Ray::new(rec.p, direction);

    return (true, attenuation, scattered);
  }
}