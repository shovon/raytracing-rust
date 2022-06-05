use crate::lambertian::Lambertian;
use crate::material::MaterialType;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord {
  pub t: f32,
  pub p: crate::vec3::Vec3,
  pub normal: crate::vec3::Vec3,
  pub material: MaterialType,
}

impl HitRecord {
  pub fn empty() -> HitRecord {
    HitRecord {
      t: 0.0,
      p: Vec3::zero(),
      normal: Vec3::zero(),
      material: MaterialType::LambertianMat(Lambertian::default()),
    }
  }
}

pub trait Hitable {
  fn hit(&self, r: crate::ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
