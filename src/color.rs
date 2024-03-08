use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

impl Color {
  pub fn to_color_string(&self, samples_per_pixel: i64) -> String {
    let scale = 1.0 / (samples_per_pixel as f64);

    let r = Color::linear_to_gamma(self.x * scale);
    let g = Color::linear_to_gamma(self.y * scale);
    let b = Color::linear_to_gamma(self.z * scale);

    let intensity = Interval::new_from_range(0.000, 0.999);

    format!("{} {} {}",
      256.0 * intensity.clamp(r),
      256.0 * intensity.clamp(g),
      256.0 * intensity.clamp(b)
    )
  }

  fn linear_to_gamma(x: f64) -> f64 {
    x.sqrt()
  }
}