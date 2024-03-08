use crate::{hittable::{HitRecord, Hittable}, interval::Interval, ray::Ray};

pub struct HittableList {
  pub objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
  pub fn new(object: Box<dyn Hittable>) -> HittableList {
    HittableList {
      objects: vec![object]
    }
  }
}

impl Hittable for &HittableList {
  fn hit(&self, ray: &Ray, ray_t: &Interval) -> (bool, Option<HitRecord>) {
    let mut hit_anything = false;
    let mut closest_so_far = ray_t.max;
    let mut rec: Option<HitRecord> = None;

    for object in &self.objects {
      let interval = Interval::new_from_range(ray_t.min, closest_so_far);
      let (hit, temp_rec) = object.hit(ray, &interval);
      if hit {
        let uw_temp_rec = temp_rec.unwrap();
        hit_anything = true;
        closest_so_far = uw_temp_rec.t;
        rec = Some(uw_temp_rec);
      }
    }

    return (hit_anything, rec);
  }
}