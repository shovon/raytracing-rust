mod camera;
mod hit_record;
mod hitable_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hit_record::Hitable;
use hitable_list::HitableList;
use ray::Ray;
use vec3::Vec3;

fn color<T: Hitable>(r: ray::Ray, world: &T) -> vec3::Vec3 {
  let mut rec = hit_record::HitRecord::empty();

  if world.hit(r, 0.001, f32::MAX, &mut rec) {
    let target = rec.p.add(rec.normal).add(Vec3::random_in_unit_sphere());
    return color(Ray::new(rec.p, target.sub(rec.p)), world).scalar_mul(0.5);
  }

  let unit_direction = r.direction().unit_vector();
  let t = (unit_direction.e1 + 1.0) * 0.5;

  Vec3::new(1.0, 1.0, 1.0)
    .scalar_mul(1.0 - t)
    .add(Vec3::new(0.5, 0.7, 1.0).scalar_mul(t))
}

fn main() {
  let nx = 800;
  let ny = 400;
  let ns = 100;
  println!("P3");
  println!("{0} {1}", nx, ny);
  println!("255\n");

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

  let cam = Camera::default();

  for j in (0..ny).rev() {
    for i in 0..nx {
      let mut col = Vec3::zero();
      for _ in 0..ns {
        let u = (i as f32 + rand::random::<f32>()) / nx as f32;
        let v = (j as f32 + rand::random::<f32>()) / ny as f32;

        let r = cam.get_ray(u, v);
        col = col.add(color(r, &mut world));
      }

      col = col.scalar_div(ns as f32);
      col = Vec3::new(col.e0.sqrt(), col.e1.sqrt(), col.e2.sqrt());

      let ir = (255.99 * col.e0) as i16;
      let ig = (255.99 * col.e1) as i16;
      let ib = (255.99 * col.e2) as i16;
      println!("{0} {1} {2}", ir, ig, ib);
    }
  }
}
