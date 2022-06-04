mod vec3;

fn main() {
  let nx = 200;
  let ny = 100;
  println!("P3");
  println!("{0} {1}", nx, ny);
  println!("255\n");
  
  for j in (0..ny).rev() {
    for i in 0..nx {
      let col = vec3::Vec3 {
        e0: (i as f32) / (nx as f32),
        e1: (j as f32) / (ny as f32),
        e2: 0.2,
      };
      let ir = (255.99 * col.e0) as i16;
      let ig = (255.99 * col.e1) as i16;
      let ib = (255.99 * col.e2) as i16;
      println!("{0} {1} {2}", ir, ig, ib);
    }
  }
}