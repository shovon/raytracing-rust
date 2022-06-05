use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]

pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
  pub u: Vec3,
  pub v: Vec3,
  pub w: Vec3,
  pub lens_radius: f32,
}

impl Camera {
  // pub fn default() -> Camera {
  //   Camera {
  //     lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
  //     horizontal: Vec3::new(4.0, 0.0, 0.0),
  //     vertical: Vec3::new(0.0, 2.0, 0.0),
  //     origin: Vec3::new(0.0, 0.0, 0.0),
  //   }
  // }

  pub fn new(
    lookfrom: Vec3,
    lookat: Vec3,
    vup: Vec3,
    vfov: f32,
    aspect: f32,
    aperture: f32,
    focus_dist: f32,
  ) -> Camera {
    let origin = lookfrom;

    let u: Vec3;
    let v: Vec3;
    let w: Vec3;
    let theta = vfov * std::f32::consts::PI / 180.0;
    let half_height = (theta / 2.0).tan();
    let half_width = aspect * half_height;
    w = lookfrom.sub(lookat).unit_vector();
    u = vup.cross(w);
    v = w.cross(u);
    // let mut lower_left_corner = Vec3::new(-half_width, -half_height, -1.0);
    let lower_left_corner = origin
      .sub(u.scalar_mul(half_width * focus_dist))
      .sub(v.scalar_mul(half_height * focus_dist))
      .sub(w.scalar_mul(focus_dist));

    Camera {
      lower_left_corner: lower_left_corner,
      horizontal: u.scalar_mul(2.0 * half_width * focus_dist),
      vertical: v.scalar_mul(2.0 * half_height * focus_dist),
      origin: lookfrom,
      u: u,
      v: v,
      w: w,
      lens_radius: aperture / 2.0,
    }
  }

  pub fn get_ray(self, u: f32, v: f32) -> Ray {
    let rd = Vec3::random_in_unit_sphere().scalar_mul(self.lens_radius);
    let offset = self.u.scalar_mul(rd.e0).add(self.v.scalar_mul(rd.e1));
    Ray {
      a: self.origin.add(offset),
      b: self
        .lower_left_corner
        .add(self.horizontal.scalar_mul(u))
        .add(self.vertical.scalar_mul(v))
        .sub(self.origin)
        .sub(offset),
    }
  }
}
