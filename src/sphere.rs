use crate::{hittable::{HitRecord, Hittable}, interval::Interval, material::Material, ray::Ray, vec3::Point3};

pub struct Sphere {
  pub center: Point3,
  pub radius: f64,
  pub material: Box<dyn Material>,
}

impl Sphere {
  pub fn new(center: Point3, radius: f64, material: Box<dyn Material>) -> Sphere {
    Sphere {
      center,
      radius,
      material,
    }
  }
}

impl Hittable for Sphere {
  fn hit<'a>(&'a self, ray: &Ray, ray_t: &Interval) -> (bool, Option<HitRecord>) {
    let oc = ray.origin - &self.center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(&ray.direction);
    let c = oc.length_squared() - &self.radius * &self.radius;

    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 { 
      return (false, None);
    };

    let sqrtd = discriminant.sqrt();

    // Find the nearest root that lies in the acceptable range
    let mut root = (-half_b - sqrtd) / a;
    if !ray_t.surrounds(root) {
      root = (-half_b + sqrtd) / a;
      if !ray_t.surrounds(root) {
        return (false, None);
      }
    }

    let t = root;
    let p = ray.at(t);
    let normal = (&p - &self.center) / self.radius;

    let rec = HitRecord {
      p,
      t,
      material: &self.material,
      normal,
      front_face: false,
    };

    let outward_normal = (rec.p - self.center) / self.radius;
    let rec = rec.set_face_normal(ray, outward_normal);

    return (true, Some(rec));
  }
}