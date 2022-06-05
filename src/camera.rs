use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]

pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
}

impl Camera {
  pub fn default() -> Camera {
    Camera {
      lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
      horizontal: Vec3::new(4.0, 0.0, 0.0),
      vertical: Vec3::new(0.0, 2.0, 0.0),
      origin: Vec3::new(0.0, 0.0, 0.0),
    }
  }

  pub fn new(vfov: f32, aspect: f32) -> Camera {
    let theta = vfov * std::f32::consts::PI / 180.0;
    let half_height = (theta / 2.0).tan();
    let half_width = aspect * half_height;
    Camera {
      lower_left_corner: Vec3::new(-half_width, -half_height, -1.0),
      horizontal: Vec3::new(2.0 * half_width, 0.0, 0.0),
      vertical: Vec3::new(0.0, 2.0 * half_height, 0.0),
      origin: Vec3::new(0.0, 0.0, 0.0),
    }
  }

  pub fn get_ray(self, u: f32, v: f32) -> Ray {
    Ray {
      a: self.origin,
      b: self
        .lower_left_corner
        .add(self.horizontal.scalar_mul(u))
        .add(self.vertical.scalar_mul(v))
        .sub(self.origin),
    }
  }
}
