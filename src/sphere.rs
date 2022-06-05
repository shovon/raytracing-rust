use crate::hit_record::HitRecord;
use crate::hit_record::Hitable;
use crate::material::MaterialType;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
  pub material: MaterialType,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f32, material: MaterialType) -> Sphere {
    return Sphere {
      center: center,
      radius: radius,
      material: material,
    };
  }
}

impl Hitable for Sphere {
  fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
    let oc = r.origin().sub(self.center);
    let a = r.direction().dot(r.direction());
    let b = oc.dot(r.direction());
    let c = oc.dot(oc) - self.radius * self.radius;
    let discriminant = b * b - a * c;
    if discriminant > 0.0 {
      let mut temp = (-b - (b * b - a * c).sqrt()) / a;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = r.point_at_parameter(rec.t);
        rec.normal = (rec.p.sub(self.center)).scalar_div(self.radius);
        rec.material = self.material.clone();
        return true;
      }
      temp = (-b + (b * b - a * c).sqrt()) / a;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = r.point_at_parameter(rec.t);
        rec.normal = (rec.p.sub(self.center)).scalar_div(self.radius);
        rec.material = self.material.clone();
        return true;
      }
    }
    return false;
  }
}
