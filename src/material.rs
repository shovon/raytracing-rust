use crate::dialectric::Dialectric;
use crate::hit_record::HitRecord;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
  fn scatter(self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

#[derive(Copy, Clone)]
pub enum MaterialType {
  LambertianMat(Lambertian),
  MetalMat(Metal),
  DialectricMat(Dialectric),
}

impl Material for MaterialType {
  fn scatter(self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
    match self {
      MaterialType::LambertianMat(l) => l.scatter(r_in, rec, attenuation, scattered),
      MaterialType::MetalMat(m) => m.scatter(r_in, rec, attenuation, scattered),
      MaterialType::DialectricMat(d) => d.scatter(r_in, rec, attenuation, scattered),
    }
  }
}
