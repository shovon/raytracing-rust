mod camera;
mod dialectric;
mod helpers;
mod hit_record;
mod hitable_list;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use dialectric::Dialectric;
use hit_record::Hitable;
use hitable_list::HitableList;
use lambertian::Lambertian;
use material::Material;
use material::MaterialType;
use metal::Metal;
use ray::Ray;
use vec3::Vec3;

fn color<T: Hitable>(r: ray::Ray, world: &T, depth: u8) -> vec3::Vec3 {
  let mut rec = hit_record::HitRecord::empty();

  if world.hit(r, 0.001, f32::MAX, &mut rec) {
    let mut scattered = Ray::new(Vec3::zero(), Vec3::zero());
    let mut attenuation = Vec3::zero();

    if depth < 50
      && rec
        .material
        .scatter(r, rec, &mut attenuation, &mut scattered)
    {
      return color(scattered, world, depth + 1).mul(attenuation);
    }

    return Vec3::zero();
  }

  let unit_direction = r.direction().unit_vector();
  let t = (unit_direction.e1 + 1.0) * 0.5;

  Vec3::new(1.0, 1.0, 1.0)
    .scalar_mul(1.0 - t)
    .add(Vec3::new(0.5, 0.7, 1.0).scalar_mul(t))
}

fn random_scene() -> HitableList {
  let mut list: Vec<Box<dyn crate::hit_record::Hitable>> = Vec::new();
  list.push(Box::new(sphere::Sphere::new(
    Vec3::new(0.0, -1000.0, 0.0),
    1000.0,
    MaterialType::LambertianMat(Lambertian {
      albedo: Vec3::new(0.5, 0.5, 0.5),
    }),
  )));

  for a in -11..11 {
    for b in -11..11 {
      let choose_mat = rand::random::<f32>();
      let center = Vec3::new(
        (a as f32) + 0.9 * rand::random::<f32>(),
        0.2,
        (b as f32) + 0.9 * rand::random::<f32>(),
      );
      if center.sub(Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
        if choose_mat < 0.8 {
          list.push(Box::new(sphere::Sphere::new(
            center,
            0.2,
            MaterialType::LambertianMat(Lambertian {
              albedo: Vec3::new(
                rand::random::<f32>() * rand::random::<f32>(),
                rand::random::<f32>() * rand::random::<f32>(),
                rand::random::<f32>() * rand::random::<f32>(),
              ),
            }),
          )));
        } else if choose_mat < 0.95 {
          list.push(Box::new(sphere::Sphere::new(
            center,
            0.2,
            MaterialType::MetalMat(Metal {
              fuzz: 0.5 * rand::random::<f32>(),
              albedo: Vec3::new(
                0.5 * (1.0 + rand::random::<f32>()),
                0.5 * (1.0 + rand::random::<f32>()),
                0.5 * (1.0 + rand::random::<f32>()),
              ),
            }),
          )));
        } else {
          list.push(Box::new(sphere::Sphere::new(
            center,
            0.2,
            MaterialType::DialectricMat(Dialectric { ref_idx: 1.5 }),
          )));
        }
      }
    }
  }

  list.push(Box::new(sphere::Sphere::new(
    Vec3::new(0.0, 1.0, 0.0),
    1.0,
    MaterialType::DialectricMat(Dialectric { ref_idx: 1.5 }),
  )));

  list.push(Box::new(sphere::Sphere::new(
    Vec3::new(-4.0, 1.0, 0.0),
    1.0,
    MaterialType::LambertianMat(Lambertian {
      albedo: Vec3::new(0.4, 0.2, 0.1),
    }),
  )));

  list.push(Box::new(sphere::Sphere::new(
    Vec3::new(4.0, 1.0, 0.0),
    1.0,
    MaterialType::MetalMat(Metal {
      fuzz: 0.0,
      albedo: Vec3::new(0.7, 0.6, 0.5),
    }),
  )));

  hitable_list::HitableList { list }
}

fn main() {
  let nx = 1024;
  let ny = 576;
  let ns = 100;
  println!("P3");
  println!("{0} {1}", nx, ny);
  println!("255\n");

  let mut world = random_scene();

  let lookfrom = Vec3::new(12.0, 2.0, 4.0);
  let lookat = Vec3::new(0.0, 1.0, 0.0);
  let dist_to_focus = lookfrom.sub(lookat).length();
  let aperture = 0.2;

  let cam = Camera::new(
    lookfrom,
    lookat,
    Vec3::new(0.0, 1.0, 0.0),
    20.0,
    (nx as f32) / (ny as f32),
    aperture,
    dist_to_focus,
  );

  for j in (0..ny).rev() {
    for i in 0..nx {
      let mut col = Vec3::zero();
      for _ in 0..ns {
        let u = (i as f32 + rand::random::<f32>()) / nx as f32;
        let v = (j as f32 + rand::random::<f32>()) / ny as f32;

        let r = cam.get_ray(u, v);
        col = col.add(color(r, &mut world, 0));
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
