use crate::helpers;
use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Metal {
  pub albedo: Vec3,
  pub fuzz: f32,
}

impl Material for Metal {
  fn scatter(self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
    let reflected = helpers::reflect(r_in.direction().unit_vector(), rec.normal);
    *scattered = Ray::new(
      rec.p,
      reflected.add(Vec3::random_in_unit_sphere().scalar_mul(self.fuzz)),
    );
    *attenuation = self.albedo;
    true
  }
}
