mod hit_record;
mod hitable_list;
mod ray;
mod sphere;
mod vec3;

use hit_record::Hitable;
use hitable_list::HitableList;
use vec3::Vec3;

fn color<T: Hitable>(r: ray::Ray, world: &T) -> vec3::Vec3 {
  let mut rec = hit_record::HitRecord::empty();
  if world.hit(r, 0.0, f32::MAX, &mut rec) {
    return Vec3::new(
      rec.normal.e0 + 1.0,
      rec.normal.e1 + 1.0,
      rec.normal.e2 + 1.0,
    )
    .scalar_mul(0.5);
  }

  let unit_direction = r.direction().unit_vector();
  let t = (unit_direction.e1 + 1.0) * 0.5;
  return Vec3::new(1.0, 1.0, 1.0)
    .scalar_mul(1.0 - t)
    .add(Vec3::new(0.5, 0.7, 1.0).scalar_mul(t));
}

fn main() {
  let nx = 200;
  let ny = 100;
  println!("P3");
  println!("{0} {1}", nx, ny);
  println!("255\n");

  let lower_left_corner = vec3::Vec3 {
    e0: -2.0,
    e1: -1.0,
    e2: -1.0,
  };
  let horizontal = vec3::Vec3 {
    e0: 4.0,
    e1: 0.0,
    e2: 0.0,
  };
  let vertical = vec3::Vec3 {
    e0: 0.0,
    e1: 2.0,
    e2: 0.0,
  };
  let origin = vec3::Vec3 {
    e0: 0.0,
    e1: 0.0,
    e2: 0.0,
  };

  let mut list: Vec<Box<dyn crate::hit_record::Hitable>> = Vec::new();
  list.push(Box::new(sphere::Sphere::new(
    Vec3::new(0.0, 0.0, -1.0),
    0.5,
  )));
  list.push(Box::new(sphere::Sphere::new(
    Vec3::new(0.0, -100.5, -1.0),
    100.0,
  )));
  let mut world = HitableList { list: list };
  for j in (0..ny).rev() {
    for i in 0..nx {
      let u = (i as f32) / (nx as f32);
      let v = (j as f32) / (ny as f32);

      let r = ray::Ray {
        a: origin,
        b: lower_left_corner
          .add(horizontal.scalar_mul(u))
          .add(vertical.scalar_mul(v)),
      };
      let col = color(r, &mut world);

      let ir = (255.99 * col.e0) as i16;
      let ig = (255.99 * col.e1) as i16;
      let ib = (255.99 * col.e2) as i16;
      println!("{0} {1} {2}", ir, ig, ib);
    }
  }
}
