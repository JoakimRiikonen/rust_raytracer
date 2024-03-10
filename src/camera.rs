use std::fs;

use crate::{color::Color, common::Common, hittable::Hittable, interval::Interval, ray::Ray, vec3::{Point3, Vec3}};

pub struct Camera<'a> {
  aspect_ratio: f64,
  image_width: i64,
  samples_per_pixel: i64,
  max_depth: i64,
  vfov: f64,
  look_from: Point3,
  look_at: Point3,
  vup: Vec3,
  filename: &'a str,
}

struct CameraComputedSettings {
  image_height: i64,
  center: Point3,
  pixel00_loc: Point3,
  pixel_delta_u: Vec3,
  pixel_delta_v: Vec3,
}

impl Camera<'_> {
  pub fn new<'a>(aspect_ratio: f64, image_width: i64, samples_per_pixel: i64,
    max_depth: i64, vfov: f64, look_from: Point3,
    look_at: Point3, vup: Vec3, filename: &'a str) -> Camera {
    Camera {
      aspect_ratio,
      image_width,
      samples_per_pixel,
      max_depth,
      vfov,
      look_from,
      look_at,
      vup,
      filename,
    }
  }

  pub fn render<'a>(&self, world: Box<dyn Hittable + 'a>) {
    let settings = self.init_settings();

    let mut output = format!("P3\n{} {}\n255\n", self.image_width, settings.image_height);

    for j in 0..settings.image_height {
        print!("\rScanlines remaining: {}  ", settings.image_height-j);
        for i in 0..self.image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..self.samples_per_pixel {
              let r = Camera::get_ray(i, j, &settings);
              pixel_color = pixel_color + Camera::ray_color(&r, self.max_depth,&world);
            }

            let row = format!("{}\n", pixel_color.to_color_string(self.samples_per_pixel));

            output.push_str(&row);
        }
    }

    print!("\rDone!.                    \n");
    _ = fs::write(self.filename, output);
  }

  fn init_settings(&self) -> CameraComputedSettings {
    // Calculate the image height, and ensure it't at least 1
    let image_height = (self.image_width as f64 / self.aspect_ratio) as i64;
    let image_height = if image_height < 1 { 1 } else { image_height };

    let center = self.look_from;

    let focal_length = (self.look_from - self.look_at).length();
    let theta = Common::degrees_to_radians(self.vfov);
    let h = (theta / 2.0).tan();
    let viewport_height = 2.0 * h * focal_length;
    let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);

    let w = (self.look_from - self.look_at).unit_vector();
    let u = self.vup.cross(&w).unit_vector();
    let v = w.cross(&u);

    // Calculate vectors across the horizontal and down the vertical viewport edges
    let viewport_u = u  * viewport_width;
    let viewport_v = -v * viewport_height;

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = &viewport_u / self.image_width as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    // Calculate location of upper-left pixel
    let viewport_upper_left = &center
        - &(w * focal_length)
        - &viewport_u / 2.0
        - &viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (&pixel_delta_u + &pixel_delta_v) * 0.5;

    CameraComputedSettings {
      image_height,
      center: center,
      pixel00_loc,
      pixel_delta_u,
      pixel_delta_v,
    }
  }

  fn get_ray(i: i64, j: i64, settings: &CameraComputedSettings) -> Ray {
    let pixel_center = &settings.pixel00_loc + &(&settings.pixel_delta_u * i as f64) + (&settings.pixel_delta_v * j as f64);
    let pixel_sample = pixel_center + Camera::pixel_sample_square(&settings);
    
    let ray_origin = settings.center;
    let ray_direction = pixel_sample - ray_origin;

    return Ray::new(ray_origin, ray_direction);
  }

  fn pixel_sample_square(settings: &CameraComputedSettings) -> Vec3 {
    let px = -0.5 + Common::random_float();
    let py = -0.5 + Common::random_float();
    return (settings.pixel_delta_u * px) + (settings.pixel_delta_v * py);
  }

  fn ray_color<'a>(ray: &Ray, depth: i64, world: &Box<dyn Hittable + 'a>) -> Color {
    if depth <= 0 {
      return Color::new(0.0, 0.0, 0.0);
    }

    let interval = Interval::new_from_range(0.001, Common::INFINITY);
    let (hit, rec) = world.hit(ray, &interval);
    if hit {
      let uw_rec = rec.unwrap();
      let (scatter, attenuation, ray_scattered) = uw_rec.material.scatter(&ray, &uw_rec);
      if scatter {
        let color = Camera::ray_color(&ray_scattered, depth-1, world);
        return attenuation * color;
      } 

      return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
  }
}