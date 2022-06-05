use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Lambertian {
  pub albedo: Vec3,
}

impl Lambertian {
  pub fn default() -> Lambertian {
    Lambertian {
      albedo: Vec3::zero(),
    }
  }
}

impl Material for Lambertian {
  fn scatter(
    self,
    _r_in: Ray,
    rec: HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
  ) -> bool {
    let target = rec.p.add(rec.normal).add(Vec3::random_in_unit_sphere());
    *scattered = Ray::new(rec.p, target.sub(rec.p));
    *attenuation = self.albedo;
    true
  }
}
