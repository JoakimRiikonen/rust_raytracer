use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
  pub fn to_color_string(&self) -> String {
    format!("{} {} {}", 255.999 * self.x, 255.999 * self.y, 255.999 * self.z)
  }
}