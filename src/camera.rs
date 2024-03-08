use std::fs;

use crate::{color::Color, common::Common, hittable::Hittable, interval::Interval, ray::Ray, vec3::{Point3, Vec3}};

pub struct Camera<'a> {
  aspect_ratio: f64,
  image_width: i64,
  samples_per_pixel: i64,
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
  pub fn new<'a>(aspect_ratio: f64, image_width: i64, samples_per_pixel: i64, filename: &'a str) -> Camera {
    Camera {
      aspect_ratio,
      image_width,
      samples_per_pixel,
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
              pixel_color = pixel_color + Camera::ray_color(&r, &world);
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

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);
    let center = Point3::new(0.0, 0.0, 0.0);

    // Calculate vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = &viewport_u / self.image_width as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    // Calculate location of upper-left pixel
    let viewport_upper_left = &center
        - &Vec3::new(0.0, 0.0, focal_length)
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

  fn ray_color<'a>(ray: &Ray, world: &Box<dyn Hittable + 'a>) -> Color {
      let interval = Interval::new_from_range(0.0, Common::INFINITY);
      let (hit, rec) = world.hit(ray, &interval);
      if hit {
          return rec.unwrap().normal + Color::new(1.0, 1.0, 1.0) * 0.5;
      }

      let unit_direction = ray.direction.unit_vector();
      let a = 0.5 * (unit_direction.y + 1.0);
      Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
  }
}