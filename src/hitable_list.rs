use crate::hit_record::HitRecord;

pub struct HitableList {
  pub list: Vec<Box<dyn crate::hit_record::Hitable>>,
}

impl crate::hit_record::Hitable for HitableList {
  fn hit(&self, r: crate::ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
    let mut temp_rec = HitRecord::empty();
    let mut hit_anything = false;
    let mut closest_so_far = t_max;
    for item in &self.list {
      if item.hit(r, t_min, closest_so_far, &mut temp_rec) {
        hit_anything = true;
        closest_so_far = temp_rec.t;
        *rec = temp_rec;
      }
    }
    hit_anything
  }
}
