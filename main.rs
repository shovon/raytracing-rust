mod vec3;
mod ray;

fn hit_sphere(center: vec3::Vec3, radius: f32, r: ray::Ray) -> f32 {
  let oc = r.origin().sub(center);
  let a = r.direction().dot(r.direction());
  let b = 2.0 * oc.dot(r.direction());
  let c = oc.dot(oc) - radius * radius;
  let discriminant = b*b - 4.0*a*c;
  if discriminant < 0.0 {
    return -1.0;
  }
  return (-b - discriminant.sqrt()) / (2.0*a);
}

fn color(r: ray::Ray) -> vec3::Vec3 {
  let t = hit_sphere(vec3::Vec3{e0: 0.0, e1: 0.0, e2: -1.0}, 0.5, r);
  if t > 0.0 {
    let n = r.point_at_parameter(t).sub(vec3::Vec3{e0:0.0,e1:0.0,e2:-1.0});
    return (vec3::Vec3{e0: n.e0+1.0, e1: n.e1+1.0, e2: n.e2+1.0}).scalar_mul(0.5);
  }
  let unit_direction = r.direction().unit_vector();
  let t = 0.5 * (unit_direction.e1 + 1.0);
  let evec = vec3::Vec3{e0: 1.0, e1: 1.0, e2: 1.0};
  return evec.scalar_mul(1.0 - t).add((vec3::Vec3{e0: 0.5, e1: 0.7, e2: 1.0}).scalar_mul(t));
}

fn main() {
  let nx = 200;
  let ny = 100;
  println!("P3");
  println!("{0} {1}", nx, ny);
  println!("255\n");

  let lower_left_corner = vec3::Vec3{e0: -2.0, e1: -1.0, e2: -1.0};
  let horizontal = vec3::Vec3{e0: 4.0, e1: 0.0, e2: 0.0};
  let vertical = vec3::Vec3{e0: 0.0, e1: 2.0, e2: 0.0};
  let origin = vec3::Vec3{e0: 0.0, e1: 0.0, e2: 0.0};
  
  for j in (0..ny).rev() {
    for i in 0..nx {
      let u = (i as f32) / (nx as f32);
      let v = (j as f32) / (ny as f32);

      let r = ray::Ray{a: origin, b: lower_left_corner.add(horizontal.scalar_mul(u)).add(vertical.scalar_mul(v))};
      let col = color(r);

      let ir = (255.99 * col.e0) as i16;
      let ig = (255.99 * col.e1) as i16;
      let ib = (255.99 * col.e2) as i16;
      println!("{0} {1} {2}", ir, ig, ib);
    }
  }
}
