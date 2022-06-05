use crate::helpers;
use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
  let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
  r0 = r0 * r0;
  r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
  let uv = v.unit_vector();
  let dt = uv.dot(n);
  let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
  if discriminant > 0.0 {
    *refracted = uv
      .sub(n.scalar_mul(dt))
      .scalar_mul(ni_over_nt)
      .sub(n.scalar_mul(discriminant.sqrt()));
    return true;
  }

  false
}

#[derive(Copy, Clone)]
pub struct Dialectric {
  pub ref_idx: f32,
}

impl Material for Dialectric {
  fn scatter(self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
    let outward_normal: Vec3;
    let reflected = helpers::reflect(r_in.direction(), rec.normal);
    let ni_over_nt: f32;
    *attenuation = Vec3::new(1.0, 1.0, 1.0);
    let mut refracted = Vec3::new(0.0, 0.0, 0.0);
    let reflect_prob: f32;
    let cosine: f32;
    if r_in.direction().dot(rec.normal) > 0.0 {
      outward_normal = rec.normal.scalar_mul(-1.0);
      ni_over_nt = self.ref_idx;
      cosine = self.ref_idx * r_in.direction().dot(rec.normal) / r_in.direction().length();
    } else {
      outward_normal = rec.normal;
      ni_over_nt = 1.0 / self.ref_idx;
      cosine = -r_in.direction().dot(rec.normal) / r_in.direction().length();
    }
    if refract(r_in.direction(), outward_normal, ni_over_nt, &mut refracted) {
      reflect_prob = schlick(cosine, self.ref_idx);
    } else {
      *scattered = Ray::new(rec.p, reflected);
      reflect_prob = 1.0;
    }
    if rand::random::<f32>() < reflect_prob {
      *scattered = Ray::new(rec.p, reflected);
    } else {
      *scattered = Ray::new(rec.p, refracted);
    }
    true
  }
}
